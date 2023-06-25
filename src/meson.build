# Configuration file
conf = configuration_data()
conf.set_quoted('APP_ID', application_id)
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('VERSION', version)

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src', 'config.rs'),
  join_paths(meson.project_source_root(), 'src', 'config.rs'),
  check: true
)

# Setup cargo
cargo = find_program('cargo')
cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]
cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'target' ]
cargo_options += [ '--release' ]

cargo_release = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'target' / 'release' / meson.project_name(), '@OUTPUT@',
  ]
)

