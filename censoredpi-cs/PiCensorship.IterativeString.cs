using System.Text;

namespace CensoredPi;

public partial class PiCensorship
{
    /// <summary>
    /// Censors any digit of pi that is smaller than the previous digit.
    /// </summary>
    /// <returns>Count of censored digits.</returns>
    public static async ValueTask<int> IterativeStringWriteCensoredDigitsOfPiAsUtf8BytesAsync(string π, Stream output, CancellationToken cancel)
    {
        // The 3. is always the same, so we can just write it out.
        var prefix = π[0..2];

        // The long tail of 14... is what we actually censor.
        // We do not extract it to a variable here to avoid making a copy of the data.

        // Business logic for consecutive suffix numbers:
        // * if the number gets bigger, we allow it.
        // * if the number is equal, we allow it.
        // * if the number gets smaller, we censor it.
        // * the first number is allowed.
        // e.g. 3.14*59*6**589*9**38*6*6**3...

        // We return how many numbers we censored.
        var censoredNumberCount = 0;

        // We keep track of the previous number as a character and just ASCII-compare each one.
        // Start with zero (lowest value) to ensure the first number is allowed without special cases in algorithm.
        char previous = '0';

        // Skip the first two characters (the prefix).
        var censoredSuffixChars = π.Skip(2).Select(c =>
        {
            var isSmallerThanPrevious = c < previous;
            previous = c;

            // If the number is smaller than the previous, we censor it.
            if (isSmallerThanPrevious)
            {
                censoredNumberCount++;
                return '*';
            }

            return c;
        }).ToArray();

        await output.WriteAsync(Encoding.UTF8.GetBytes(prefix), cancel);
        await output.WriteAsync(Encoding.UTF8.GetBytes(censoredSuffixChars), cancel);

        return censoredNumberCount;
    }
}
