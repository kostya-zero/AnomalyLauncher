fn main() {
    // Prevent building on non-Windows machines.
    if !cfg!(target_os = "windows") {
        panic!("------\nAnomaly Launcher should be built only on Windows!\n------");
    }

    let mut res = winresource::WindowsResource::new();

    res.set("FileVersion", "1.0.0.0")
        .set("ProductVersion", "1.0.0.0")
        .set("ProductName", "Anomaly Launcher")
        .set(
            "FileDescription",
            "An alternative launcher for S.T.A.L.K.E.R Anomaly.",
        )
        .set("OriginalFilename", "AnomalyLauncher.exe")
        .set("LegalCopyright", "Copyright © 2025 Konstantin Zhigaylo")
        .set("CompanyName", "Konstantin Zhigaylo")
        .set("InternalName", "AnomalyLauncher");

    // Set language to English
    res.set_language(0x0409);

    res.set_icon("assets/icon.ico");
    res.compile()
        .expect("Failed to run the Windows resource compiler (rc.exe)");
}
