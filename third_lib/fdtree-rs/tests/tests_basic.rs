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
            assert_eq!(region.size.unwrap(), 0x10000000);
        } else {
            assert_eq!(region.starting_address as usize, 0xa_0000_0000);
            assert_eq!(region.size.unwrap(), 0x20000000);
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
                assert_eq!(region.size.unwrap(), 0x10000000);
            }
            if idn == 1 && idx == 0 {
                assert_eq!(region.starting_address as usize, 0x90000000);
                assert_eq!(region.size.unwrap(), 0x10000000);
            }
        }
    }
}
