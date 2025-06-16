RUSTDOCFLAGS += -Z unstable-options --enable-index-page -D rustdoc::broken_intra_doc_links

$(if $(V), $(info RUSTDOCFLAGS = "$(RUSTDOCFLAGS)"))

