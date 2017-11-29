require 'benchmark'
require 'spec_helper'

describe ParallelSort::Integers do
  let(:input) { [5, 4, 3, 2, 1] }
  let(:output) { [1, 2, 3, 4, 5] }

  describe '.sort' do
    subject { described_class.sort(input) }

    it { is_expected.to eq(output) }
  end

  describe '.par_sort' do
    subject { described_class.par_sort(input) }

    it { is_expected.to eq(output) }
  end

  describe 'bench' do
    it 'sorts' do
      bigger_input = (1..10_000_000).to_a.reverse!

      Benchmark.bmbm do |x|
        x.report("sort_ruby") { bigger_input.sort }
        x.report("sort_rust")  { ParallelSort::Integers.sort(bigger_input) }
        x.report("par_sort_rust")  { ParallelSort::Integers.par_sort(bigger_input) }
      end
    end
  end
end
