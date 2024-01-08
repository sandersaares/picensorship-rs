using BenchmarkDotNet.Attributes;

namespace CensoredPi;

[MemoryDiagnoser]
//[DisassemblyDiagnoser(maxDepth: 4)]
//[EventPipeProfiler(BenchmarkDotNet.Diagnosers.EventPipeProfile.GcVerbose)]
public partial class PiCensorship
{
    [Benchmark]
    public async Task IterativeString()
    {
        await IterativeStringWriteCensoredDigitsOfPiAsUtf8BytesAsync(DigitsOfPi.Pi50KAsString, Stream.Null, CancellationToken.None);
    }

    [Benchmark]
    public async Task IterativeBytes()
    {
        await IterativeWriteCensoredDigitsOfPiAsUtf8BytesAsync(DigitsOfPi.Pi50KAsUtf8, Stream.Null, CancellationToken.None);
    }

    [Benchmark(Baseline = true)]
    public async Task InPlaceBytes()
    {
        await InPlaceWriteCensoredDigitsOfPiAsUtf8BytesAsync(DigitsOfPi.Pi50KAsUtf8, Stream.Null, CancellationToken.None);
    }

    [Benchmark]
    public void InPlaceBytesSync()
    {
        InPlaceWriteCensoredDigitsOfPiAsUtf8Bytes(DigitsOfPi.Pi50KAsUtf8, Stream.Null);
    }
}
