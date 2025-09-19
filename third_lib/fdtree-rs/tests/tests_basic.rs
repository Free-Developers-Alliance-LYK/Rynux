static DTB_DATA: &[u8] = include_bytes!("../dtb/test.dtb");

use fdtree_rs::LinuxFdt;

fn setup() -> LinuxFdt<'static> {
    LinuxFdt::new(DTB_DATA).unwrap()
}


#[test]
fn get_model() {
    let fdt = setup();
    assert_eq!(fdt.machine(), "riscv-virtio,qemu");
}

#[test]
fn chosen_node() {
    let fdt = setup();
    let chosen = fdt.chosen();
    assert_eq!(chosen.bootargs().unwrap(), "console=ttyS0");
    assert_eq!(chosen.stdout().unwrap().node.name, "uart@10000000");
    assert_eq!(chosen.stdout().unwrap().options.unwrap(), "115200n8");

    let usable_memory_range = chosen.usable_mem_region().unwrap();

    let mut cnt: usize = 0;
    for region in usable_memory_range {
        cnt +=1 ;
        if cnt == 1 {
            assert_eq!(region.starting_address as usize, 0x9_f000_0000);
            assert_eq!(region.size, 0x10000000);
        } else {
            assert_eq!(region.starting_address as usize, 0xa_0000_0000);
            assert_eq!(region.size, 0x20000000);
        }
    };
}

#[test]
fn memory_node() {
    let fdt = setup();
    assert_eq!(fdt.mem_nodes().count(), 2);
    for (idn, node) in fdt.mem_nodes().enumerate() {
        assert_eq!(1, node.regions().unwrap().count());
        for (idx, region) in node.regions().unwrap().enumerate() {
            if idn == 0 && idx == 0 {
                assert_eq!(region.starting_address as usize, 0x80000000);
                assert_eq!(region.size, 0x10000000);
            }
            if idn == 1 && idx == 0 {
                assert_eq!(region.starting_address as usize, 0x90000000);
                assert_eq!(region.size, 0x10000000);
            }
        }
    }
}

#[test]
fn linux_reserved_memory() {
    let fdt = setup();
    let reserved = fdt.linux_reserved_memory().unwrap();
    assert_eq!(reserved.valid_reserved_nodes().count(), 2);

    let mut valid_node_iter = reserved.valid_reserved_nodes();
    let vnode1 = valid_node_iter.next().unwrap();
    assert_eq!(vnode1.node.name, "static_buf@0000000080000000");
    assert_eq!(vnode1.nomap(), false);

    let mut vreg1_iter = vnode1.regions();
    assert_eq!(vreg1_iter.clone().count(), 1);
    let vreg1_0 = vreg1_iter.next().unwrap();
    assert_eq!(vreg1_0.starting_address as usize, 0x80000000);
    assert_eq!(vreg1_0.size, 0x2000000);

    let vnode2 = valid_node_iter.next().unwrap();
    assert_eq!(vnode2.node.name, "secure_carveout@0000000090000000");
    assert_eq!(vnode2.nomap(), true);

    let mut vreg2_iter = vnode2.regions();
    assert_eq!(vreg2_iter.clone().count(), 2);
    let vreg2_0 = vreg2_iter.next().unwrap();
    assert_eq!(vreg2_0.starting_address as usize, 0x90000000);
    assert_eq!(vreg2_0.size, 0x1000000);
    let vreg2_1 = vreg2_iter.next().unwrap();
    assert_eq!(vreg2_1.starting_address as usize, 0x91000000);
    assert_eq!(vreg2_1.size, 0x1000000);

}

#[test]
fn linux_reserved_memory_dynamic() {
    let fdt = setup();
    let reserved = fdt.linux_reserved_memory().unwrap();
    assert_eq!(reserved.dynamic_nodes().count(), 2);

    let mut dyn_node_iter = reserved.dynamic_nodes();
    let dyn_node1 = dyn_node_iter.next().unwrap();
    assert_eq!(dyn_node1.node.name, "dyn_pool");
    assert_eq!(dyn_node1.size(), 0x4000000);
    assert_eq!(dyn_node1.alignment(), 0x200000);
    assert_eq!(dyn_node1.nomap(), false);
    assert_eq!(dyn_node1.reusable(), false);
    assert_eq!(dyn_node1.shared_dma_pool(), false);
    assert!(dyn_node1.alloc_ranges().is_none());

    let dyn_node2 = dyn_node_iter.next().unwrap();
    assert_eq!(dyn_node2.node.name, "linux,cma");
    assert_eq!(dyn_node2.size(), 0x10000000);
    assert_eq!(dyn_node2.alignment(), 0x2000000);
    assert_eq!(dyn_node2.nomap(), false);
    assert_eq!(dyn_node2.reusable(), true);
    assert_eq!(dyn_node2.shared_dma_pool(), true);
    assert!(dyn_node2.alloc_ranges().is_none());
}

#[test]
fn sys_memory_reservations() {
    let fdt = setup();
    let mut reservations = fdt.sys_memory_reservations();
    let res_1 = reservations.next().unwrap();
    assert_eq!(res_1.address() as usize, 0x80000000);
    assert_eq!(res_1.size(), 0x1000000);

    let res_2 = reservations.next().unwrap();
    assert_eq!(res_2.address() as usize, 0x90000000);
    assert_eq!(res_2.size(), 0x100000);

    assert!(reservations.next().is_none());
}


