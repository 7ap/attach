pub fn has_permissions() -> bool {
    nix::unistd::geteuid().is_root()
}
