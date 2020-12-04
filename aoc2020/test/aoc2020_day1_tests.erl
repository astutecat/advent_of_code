-module(aoc2020_day1_tests).

-include_lib("eunit/include/eunit.hrl").

problem_1_test() ->
    {A, B} = aoc2020_day1:problem_1(),
    ?assertEqual(2020, A + B),
    ?debugFmt("Part one result: ~p~n", [A * B]).

problem_2_test() ->
    {A, B, C} = aoc2020_day1:problem_2(),
    ?assertEqual(2020, A + B + C),
    ?debugFmt("Part one result: ~p~n", [A * B * C]).
