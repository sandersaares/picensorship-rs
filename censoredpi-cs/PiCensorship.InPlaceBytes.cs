using System.Buffers;

namespace CensoredPi;

partial class PiCensorship
{
	/// <summary>
	/// Censors any digit of pi that is smaller than the previous digit.
	/// </summary>
	/// <returns>Count of censored digits.</returns>
	public static async ValueTask<int> InPlaceWriteCensoredDigitsOfPiAsUtf8BytesAsync(byte[] π, Stream output, CancellationToken cancel)
	{
		// Emit the 3. prefix as it is always the same.
		await output.WriteAsync(π.AsMemory(0..2), cancel);

		// Censor the remaining part chunk by chunk.
		var remaining = π.AsMemory(2..);

		var censoredNumberCount = 0;

		// ASP.NET Core supports zero-allocation writes up to 4KB, so limit output chunk size to this.
		const int ChunkSize = 4096;

		// We keep track of the previous number as a byte and just integer-compare each one.
		// Start with zero (lowest value) to ensure the first number is allowed without special cases in algorithm.
		byte previous = (byte)'0';

		var buffer = ArrayPool<byte>.Shared.Rent(ChunkSize);

		try
		{
			while (!remaining.IsEmpty)
			{
				var bytesInChunk = Math.Min(remaining.Length, ChunkSize);
				remaining[..bytesInChunk].CopyTo(buffer);
				remaining = remaining[bytesInChunk..];

				censoredNumberCount += CensorSuffixSpan(buffer.AsSpan(..bytesInChunk), ref previous);

				await output.WriteAsync(buffer.AsMemory(..bytesInChunk), cancel);
			}
		}
		finally
		{
			ArrayPool<byte>.Shared.Return(buffer);
		}

		return censoredNumberCount;
	}

	private static int CensorSuffixSpan(Span<byte> suffix, ref byte previous)
	{
		// Business logic for consecutive suffix numbers:
		// * if the number gets bigger, we allow it.
		// * if the number is equal, we allow it.
		// * if the number gets smaller, we censor it.
		// * the first number is allowed.
		// e.g. 3.14*59*6**589*9**38*6*6**3...

		// We return how many numbers we censored.
		var censoredNumberCount = 0;

		for (var i = 0; i < suffix.Length; i++)
		{
			var c = suffix[i];
			var isSmallerThanPrevious = c < previous;
			previous = c;

			if (isSmallerThanPrevious)
			{
				censoredNumberCount++;
                suffix[i] = (byte)'*';
			}
		}

		return censoredNumberCount;
	}
}
