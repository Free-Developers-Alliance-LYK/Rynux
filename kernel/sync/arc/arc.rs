// SPDX-License-Identifier: GPL-2.0

//! A reference-counted pointera(linux/rust/kernel/sync/arc.rs).
//! 
//!
//! This module implements a way for users to create reference-counted objects and pointers to
//! them. Such a pointer automatically increments and decrements the count, and drops the
//! underlying object when it reaches zero. It is also safe to use concurrently from multiple
//! threads.
//!
//! It is different from the standard library's [`Arc`] in a few ways:
//! 1. It is backed by the kernel's `refcount_t` type.
//! 2. It does not support weak references, which allows it to be half the size.
//! 3. It saturates the reference count instead of aborting when it goes over a threshold.
//! 4. It does not provide a `get_mut` method, so the ref counted object is pinned.
//! 5. The object in [`Arc`] is pinned implicitly.
//!
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html

//use crate::alloc::{AllocError, AllocFlags, KBox};
use core::{
    alloc::Layout,
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
};


/// A reference-counted pointer to an instance of `T`.
///
/// The reference count is incremented when new instances of [`Arc`] are created, and decremented
/// when they are dropped. When the count reaches zero, the underlying `T` is also dropped.
///
/// # Invariants
///
/// The reference count on an instance of [`Arc`] is always non-zero.
/// The object pointed to by [`Arc`] is always pinned.
///
/// # Examples
///
/// ```
/// use kernel::sync::Arc;
///
/// struct Example {
///     a: u32,
///     b: u32,
/// }
///
/// // Create a refcounted instance of `Example`.
/// let obj = Arc::new(Example { a: 10, b: 20 }, GFP_KERNEL)?;
///
/// // Get a new pointer to `obj` and increment the refcount.
/// let cloned = obj.clone();
///
/// // Assert that both `obj` and `cloned` point to the same underlying object.
/// assert!(core::ptr::eq(&*obj, &*cloned));
///
/// // Destroy `obj` and decrement its refcount.
/// drop(obj);
///
/// // Check that the values are still accessible through `cloned`.
/// assert_eq!(cloned.a, 10);
/// assert_eq!(cloned.b, 20);
///
/// // The refcount drops to zero when `cloned` goes out of scope, and the memory is freed.
/// # Ok::<(), Error>(())
/// ```
///
/// Using `Arc<T>` as the type of `self`:
///
/// ```
/// use kernel::sync::Arc;
///
/// struct Example {
///     a: u32,
///     b: u32,
/// }
///
/// impl Example {
///     fn take_over(self: Arc<Self>) {
///         // ...
///     }
///
///     fn use_reference(self: &Arc<Self>) {
///         // ...
///     }
/// }
///
/// let obj = Arc::new(Example { a: 10, b: 20 }, GFP_KERNEL)?;
/// obj.use_reference();
/// obj.take_over();
/// # Ok::<(), Error>(())
/// ```
///
/// Coercion from `Arc<Example>` to `Arc<dyn MyTrait>`:
///
/// ```
/// use kernel::sync::{Arc, ArcBorrow};
///
/// trait MyTrait {
///     // Trait has a function whose `self` type is `Arc<Self>`.
///     fn example1(self: Arc<Self>) {}
///
///     // Trait has a function whose `self` type is `ArcBorrow<'_, Self>`.
///     fn example2(self: ArcBorrow<'_, Self>) {}
/// }
///
/// struct Example;
/// impl MyTrait for Example {}
///
/// // `obj` has type `Arc<Example>`.
/// let obj: Arc<Example> = Arc::new(Example, GFP_KERNEL)?;
///
/// // `coerced` has type `Arc<dyn MyTrait>`.
/// let coerced: Arc<dyn MyTrait> = obj;
/// # Ok::<(), Error>(())
/// ```

#[repr(transparent)]
#[derive(core::marker::CoercePointee)]
pub struct Arc<T: ?Sized> {
    pub(crate) ptr: NonNull<ArcInner<T>>,
    // NB: this informs dropck that objects of type `ArcInner<T>` may be used in `<Arc<T> as
    // Drop>::drop`. Note that dropck already assumes that objects of type `T` may be used in
    // `<Arc<T> as Drop>::drop` and the distinction between `T` and `ArcInner<T>` is not presently
    // meaningful with respect to dropck - but this may change in the future so this is left here
    // out of an abundance of caution.
    //
    // See <https://doc.rust-lang.org/nomicon/phantom-data.html#generic-parameters-and-drop-checking>
    // for more detail on the semantics of dropck in the presence of `PhantomData`.
    _p: PhantomData<ArcInner<T>>,
}

#[doc(hidden)]
#[repr(C)]
pub(crate) struct ArcInner<T: ?Sized> {
    refcont: AtomicU32,
    data: T,
}

impl <T> ArcInner<T> {
    /// Creates a new [`ArcInner<T>`].
    pub(crate) const  fn new(data: T) -> Self {
        Self {
            refcont: AtomicU32::new(1),
            data,
        }
    }
}


// SAFETY: It is safe to send `Arc<T>` to another thread when the underlying `T` is `Sync` because
// it effectively means sharing `&T` (which is safe because `T` is `Sync`); additionally, it needs
// `T` to be `Send` because any thread that has an `Arc<T>` may ultimately access `T` using a
// mutable reference when the reference count reaches zero and `T` is dropped.
unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}

// SAFETY: It is safe to send `&Arc<T>` to another thread when the underlying `T` is `Sync`
// because it effectively means sharing `&T` (which is safe because `T` is `Sync`); additionally,
// it needs `T` to be `Send` because any thread that has a `&Arc<T>` may clone it and get an
// `Arc<T>` on that thread, so the thread may ultimately access `T` using a mutable reference when
// the reference count reaches zero and `T` is dropped.
unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}

/*
impl<T> Arc<T> {
    /// Constructs a new reference counted instance of `T`.
    pub fn new(contents: T, flags: Flags) -> Result<Self, AllocError> {
        // INVARIANT: The refcount is initialised to a non-zero value.
        let value = ArcInner {
            refcont: atomic::AtomicUsize::new(1),
            data: contents,
        };

        let inner = KBox::new(value, flags)?;
        let inner = KBox::leak(inner).into();

        // SAFETY: We just created `inner` with a reference count of 1, which is owned by the new
        // `Arc` object.
        Ok(unsafe { Self::from_inner(inner) })
    }
}
*/

#[inline]
fn data_offset_align(align: usize) -> usize {
    let layout = Layout::new::<ArcInner<()>>();
    layout.size() + layout.padding_needed_for(align)
}

/// Gets the offset within an `ArcInner` for the payload behind a pointer.
///
/// # Safety
///
/// The pointer must point to (and have valid metadata for) a previously
/// valid instance of T, but the T is allowed to be dropped.
unsafe fn data_offset<T: ?Sized>(ptr: *const T) -> usize {
    // Align the unsized value to the end of the ArcInner.
    // Because RcInner is repr(C), it will always be the last field in memory.

    // SAFETY: since the only unsized types possible are slices, trait objects,
    // and extern types, the input safety requirement is currently enough to
    // satisfy the requirements of align_of_val_raw; this is an implementation
    // detail of the language that must not be relied upon outside of std.
    unsafe { data_offset_align(core::mem::align_of_val_raw(ptr)) }
}

impl<T: ?Sized> Arc<T> {
    /// Constructs a new [`Arc`] from an existing [`ArcInner`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that `inner` points to a valid location and has a non-zero reference
    /// count, one of which will be owned by the new [`Arc`] instance.
    pub(crate) const unsafe fn from_inner(inner: NonNull<ArcInner<T>>) -> Self {
        // INVARIANT: By the safety requirements, the invariants hold.
        Arc {
            ptr: inner,
            _p: PhantomData,
        }
    }

    #[inline]
    fn inner(&self) -> &ArcInner<T> {
        // This unsafety is ok because while this arc is alive we're guaranteed
        // that the inner pointer is valid. Furthermore, we know that the
        // `ArcInner` structure itself is `Sync` because the inner data is
        // `Sync` as well, so we're ok loaning out an immutable pointer to these
        // contents.
        unsafe { self.ptr.as_ref() }
    }

    /// Convert the [`Arc`] into a raw pointer.
    ///
    /// The raw pointer has ownership of the refcount that this Arc object owned.
    pub fn into_raw(this: Self) -> *const T {
        let this = ManuallyDrop::new(this); // no drop
        Self::as_ptr(&*this)
    }

    /// Return a raw pointer to the data in this arc.
    pub fn as_ptr(this: &Self) -> *const T {
        let ptr: *mut ArcInner<T> = NonNull::as_ptr(this.ptr);

        // SAFETY: This cannot go through Deref::deref or RcInnerPtr::inner because
        // this is required to retain raw/mut provenance such that e.g. `get_mut` can
        // write through the pointer after the Rc is recovered through `from_raw`.
        unsafe { &raw mut (*ptr).data }
    }

    /// Recreates an [`Arc`] instance previously deconstructed via [`Arc::into_raw`].
    ///
    /// # Safety
    ///
    /// `ptr` must have been returned by a previous call to [`Arc::into_raw`]. Additionally, it
    /// must not be called more than once for each previous call to [`Arc::into_raw`].
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        // SAFETY: The caller promises that this pointer originates from a call to `into_raw` on an
        // `Arc` that is still valid.
        unsafe {
            let offset = data_offset(ptr);

            // Reverse the offset to find the original ArcInner.
            let arc_ptr = ptr.byte_sub(offset) as *mut ArcInner<T>;

            // SAFETY: By the safety requirements we know that `ptr` came from `Arc::into_raw`, so the
            // reference count held then will be owned by the new `Arc` object.
            Self::from_inner(NonNull::new_unchecked(arc_ptr))
        }
    }

    /// Compare whether two [`Arc`] pointers reference the same underlying object.
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        core::ptr::eq(this.ptr.as_ptr(), other.ptr.as_ptr())
    }

}

impl<T: ?Sized> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: By the type invariant, there is necessarily a reference to the object, so it is
        // safe to dereference it.
        &self.inner().data
    }
}

impl<T: ?Sized> AsRef<T> for Arc<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: ?Sized> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // Using a relaxed ordering is alright here, as knowledge of the
        // original reference prevents other threads from erroneously deleting
        // the object.
        //
        // As explained in the [Boost documentation][1], Increasing the
        // reference counter can always be done with memory_order_relaxed: New
        // references to an object can only be formed from an existing
        // reference, and passing an existing reference from one thread to
        // another must already provide any required synchronization.
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        let old_size = self.inner().refcont.fetch_add(1, Ordering::Relaxed);

        if old_size > u32::MAX {
            panic!("Arc overflow");
        }

        // SAFETY: We just incremented the refcount. This increment is now owned by the new `Arc`.
        unsafe { Self::from_inner(self.ptr) }
    }
}

/*
impl<T: ?Sized> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.inner().refcount.fetch_sub(1, Release) != 1 {
            return;
        }

        // This fence is needed to prevent reordering of use of the data and
        // deletion of the data. Because it is marked `Release`, the decreasing
        // of the reference count synchronizes with this `Acquire` fence. This
        // means that use of the data happens before decreasing the reference
        // count, which happens before this fence, which happens before the
        // deletion of the data.
        //
        // As explained in the [Boost documentation][1],
        //
        // > It is important to enforce any possible access to the object in one
        // > thread (through an existing reference) to *happen before* deleting
        // > the object in a different thread. This is achieved by a "release"
        // > operation after dropping a reference (any access to the object
        // > through this reference must obviously happened before), and an
        // > "acquire" operation before deleting the object.
        //
        // In particular, while the contents of an Arc are usually immutable, it's
        // possible to have interior writes to something like a Mutex<T>. Since a
        // Mutex is not acquired when it is deleted, we can't rely on its
        // synchronization logic to make writes in thread A visible to a destructor
        // running in thread B.
        //
        // Also note that the Acquire fence here could probably be replaced with an
        // Acquire load, which could improve performance in highly-contended
        // situations. See [2].
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        // [2]: (https://github.com/rust-lang/rust/pull/41714)
        acquire!(self.inner().refcont);

        // The count reached zero, we must free the memory.
        //
        // SAFETY: The pointer was initialised from the result of `KBox::leak`.
        unsafe { drop(KBox::from_raw(self.ptr.as_ptr())) };
    }
}
*/

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_arc_from_inner() {
        let innter = ArcInner {
            refcont: AtomicU32::new(1),
            data: 42,
        };
        let arc = unsafe { Arc::from_inner(NonNull::from(&innter)) };
        assert_eq!(*arc, 42);

        // test from raw
        let raw = Arc::into_raw(arc);
        let arc = unsafe { Arc::from_raw(raw) };
        assert_eq!(*arc, 42);
    }
}
