require 'thermite/fiddle'
require 'parallel_sort/version'

module ParallelSort
  # Your code goes here...
end

toplevel_dir = File.dirname(File.dirname(__FILE__))
extension_dir = File.join(toplevel_dir, 'ext', 'parallel_sort')

Thermite::Fiddle.load_module(
  'initialize_parallel_sort',
  ruby_project_path: toplevel_dir,
  cargo_project_path: extension_dir,
)
