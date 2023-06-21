use cudarc::{
    driver::{CudaDevice, DriverError, LaunchAsync, LaunchConfig},
    nvrtc::Ptx,
};

const PTX: &'static str = include_str!("add_kernel.0.0.ptx");
fn main() -> Result<(), DriverError> {
    let dev = CudaDevice::new(0)?;

    dev.load_ptx(
        Ptx::from_src(PTX),
        "add_kernel.0.0",
        &["add_kernel_0d1d2d3d"],
    )?;

    // and then retrieve the function with `get_func`
    let f = dev
        .get_func("add_kernel.0.0", "add_kernel_0d1d2d3d")
        .unwrap();

    let x = [1.0, 2.0, 3.0];
    let x = dev.htod_copy(x.into())?;
    let y = [1.0, 2.0, 3.0];
    let y = dev.htod_copy(y.into())?;
    let mut c = x.clone();

    let n = 3;
    let cfg = LaunchConfig::for_num_elems(n);
    unsafe { f.launch(cfg, (&x, &y, &mut c, n as i32)) }?;

    let c = dev.sync_reclaim(c)?;

    println!("Hello, world! {c:?}");
    Ok(())
}
