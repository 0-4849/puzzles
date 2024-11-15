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

main :-
    word_grid(
        [_11,_12],
        [_21,_22],
    ).
