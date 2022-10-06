formula "./test.ld"
formula "./test2.ld"

main of () = 
    boolTest();
    symbolTest()
end

boolTest of () =
    print 1 > 2;
    print 1 < 2;
    print 1 == 2;
    print 1 >= 2;
    print 1 <= 2;
    print 1 != 2
end

symbolTest of () =
    print 1 + 2;
    print 1 - 2;
    print 1 * 2;
    print 1 / 2;
    print 1 % 2;
    print 1 ** 2
end

{|$x|}
(||) of x =
    >= 0 => x,
    < 0  => -x
end

{$x~$y}
(~) of x y = \
    x..y

type point = (int int)
type pointS = (x: int y: int)
type pointC: pointS =
    @pub _ of x y =
        _x = x
        _y = y
    end

    @pub @property x = _x
    @pub @property y = _y
end