# Configuration file
conf = configuration_data()
conf.set_quoted('APP_ID', application_id)
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('VERSION', version + version_suffix)

# Update the application config info
configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# copy the config info into the project
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

# If the profile is development, build as debug, otherwise release
if get_option('profile') == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

# Build the actual package
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
    'cp', 'target' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)
