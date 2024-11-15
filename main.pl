:- [word_rules],
    use_module(library(clpfd)).

all_words([]).
all_words([W | WS]) :-
    word(W),
    all_words(WS).

word_grid(WS) :-
    transpose(WS, VS),
    length(WS, N),
    length(VS, N),
    all_words(WS),
    all_words(VS).

<<<<<<< HEAD
main :-
    word_grid(
        [_11,_12],
        [_21,_22],
    ).
=======
% test
>>>>>>> 9318abfb1a87b98574ab1a64bedce33d92b64e10
