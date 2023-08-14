using System.Diagnostics;
using static W33;

Debug.Assert(FaultyKeeb("string") == "rtsng");
Debug.Assert(FaultyKeeb("hello world!") == "w hllrld!");

public class W33
{
    public static string FaultyKeeb(string s) =>
        s.Aggregate("", (acc, x) => x switch
        {
            'a' or 'e' or 'i' or 'o' or 'y' or 'u' => string.Join("", acc.Reverse()),
            _ => acc + x,
        }
    );
}

