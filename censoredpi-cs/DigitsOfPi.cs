namespace CensoredPi;

internal static class DigitsOfPi
{
    public static readonly byte[] Pi50KAsUtf8 = File.ReadAllBytes("pi50k.txt");
    public static readonly string Pi50KAsString = File.ReadAllText("pi50k.txt");
}
