require 'bundler/gem_tasks'
require 'rspec/core/rake_task'
require 'thermite/tasks'

toplevel_dir = File.dirname(__FILE__)
extension_dir = File.join(toplevel_dir, 'ext', 'parallel_sort')

Thermite::Tasks.new(cargo_project_path: extension_dir)
RSpec::Core::RakeTask.new(:spec)

task test: %w(thermite:test thermite:build spec)
task default: %w(thermite:build)
