fn main() {
    #[cfg(windows)]
    {
        // Embed manifest to request administrator privilege for installer
        // <https://learn.microsoft.com/en-us/cpp/build/reference/manifest-create-side-by-side-assembly-manifest?view=msvc-170>
        println!("cargo:rustc-link-arg-bin=interception-installer=/MANIFEST:EMBED");
        // <https://learn.microsoft.com/en-us/cpp/build/reference/manifestuac-embeds-uac-information-in-manifest?view=msvc-170>
        println!(
            r"cargo:rustc-link-arg-bin=interception-installer=/MANIFESTUAC:level='requireAdministrator'"
        );
    }
}
