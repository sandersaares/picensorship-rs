namespace CensoredPi;

public partial class PiCensorship
{
    private static void ValidatePiLooksLikePi(ReadOnlyMemory<byte> π)
    {
        var span = π.Span;

        if (span[0] != '3' || span[1] != '.')
            throw new ArgumentException("π must start with '3.'", nameof(π));
    }

    private static void ValidatePiLooksLikePi(string π)
    {
        if (!π.StartsWith("3.", StringComparison.Ordinal))
            throw new ArgumentException("π must start with '3.'", nameof(π));
    }
}
