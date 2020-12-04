-module(aoc2020_day1_tests).

-include_lib("eunit/include/eunit.hrl").

problem_1_test() ->
    {A, B} = aoc2020_day1:problem_1(),
    ?assertEqual(2020, A + B).
