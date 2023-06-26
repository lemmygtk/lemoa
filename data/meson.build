desktop_conf = configuration_data()
desktop_conf.set('NAME', meson.project_name())
desktop_conf.set('ICON', application_id)
desktop_conf.set('APP_ID', application_id)
desktop_conf.set('EXEC', meson.project_name())

# Desktop file
configure_file(
  input: '@0@.desktop.in'.format(base_id),
  output: '@BASENAME@',
  install: true,
  configuration: desktop_conf,
  install_dir: applicationsdir
)

# Scalable icon
install_data(
  '@0@.svg'.format(base_id),
  install_dir: iconsdir / 'hicolor' / 'scalable' / 'apps',
  rename: '@0@.svg'.format(application_id)
)

# Symbolic icon
install_data(
  '@0@-symbolic.svg'.format(base_id),
  install_dir: iconsdir / 'hicolor' / 'symbolic' / 'apps',
  rename: '@0@-symbolic.svg'.format(application_id)
)
