overlap(XS, YS) :- append(_, ZS, XS), append(ZS, _, YS), ZS \= [].
overlap3(XS, YS) :- length(ZS, 3), append(_, ZS, XS), append(ZS, _, YS).

square3([X11, _, X13],
        [X31, _, X33],
        [X11, _, X31],
        [X31, _, X33]).

split_on_nl(X, [X]) :- \+ member(10, X).
split_on_nl(X0, X7) :-
    append(X1, X2, X0), 
    append(X4, `\n`, X1),
    split_on_nl(X4, X5),
    split_on_nl(X2, X6),
    append(X5, X6, X7).

main :-
    open('Woorden.txt', read, File),
    read_lines(File, Lines),
    close(File),
    split_on_nl(Lines, Words),
    member(X, Words),
    member(Y, Words),
    X \= Y,
    overlap3(X,Y),
    maplist(put_char, X), nl,
    maplist(put_char, Y), nl.

read_lines(File,[]) :- 
    at_end_of_stream(File).

read_lines(File,[X|L]) :-
    \+ at_end_of_stream(File),
    get_code(File,X),
    read_lines(File,L).

