-module(aoc2020_day1).

%% -compile(export_all).
-export([problem_1/0]).

problem_1() ->
    Lines = readlines("src/aoc2020_day1_input.txt"),
    ParseFun = fun(Line) -> 
                       {I, _} = string:to_integer(Line),
                       I
               end,
    FilterFun = fun(Val) -> is_number(Val) end,
    Ints = lists:filter(FilterFun, lists:map(ParseFun, Lines)),
    SortedInts = lists:sort(fun(A, B) -> A > B end, Ints),
    SortedInts.

readlines(FileName) ->
    {ok, Data} = file:read_file(FileName),
    binary:split(Data, [<<"\n">>], [global]).

