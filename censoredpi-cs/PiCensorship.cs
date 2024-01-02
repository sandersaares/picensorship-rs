using BenchmarkDotNet.Attributes;

namespace CensoredPi;

[MemoryDiagnoser]
//[EventPipeProfiler(BenchmarkDotNet.Diagnosers.EventPipeProfile.GcVerbose)]
public partial class PiCensorship
{
    [Benchmark]
    public ValueTask<int> IterativeString()
    {
        return IterativeStringWriteCensoredDigitsOfPiAsUtf8BytesAsync(DigitsOfPi.Pi50KAsString, Stream.Null, CancellationToken.None);
    }

    [Benchmark]
    public ValueTask<int> IterativeBytes()
    {
        return IterativeWriteCensoredDigitsOfPiAsUtf8BytesAsync(DigitsOfPi.Pi50KAsUtf8, Stream.Null, CancellationToken.None);
    }

    [Benchmark(Baseline = true)]
    public ValueTask<int> InPlaceBytes()
    {
        return InPlaceWriteCensoredDigitsOfPiAsUtf8BytesAsync(DigitsOfPi.Pi50KAsUtf8, Stream.Null, CancellationToken.None);
    }
}
