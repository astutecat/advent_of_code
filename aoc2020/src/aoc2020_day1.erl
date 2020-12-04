-module(aoc2020_day1).

%% -compile(export_all).
-export([problem_1/0, problem_2/0]).

problem_1() ->
    Lines = readlines("src/aoc2020_day1_input.txt"),
    ParseFun = fun(Line) -> 
                       {I, _} = string:to_integer(Line),
                       I
               end,
    FilterFun = fun(Val) -> is_number(Val) end,
    Ints = lists:filter(FilterFun, lists:map(ParseFun, Lines)),
    hd([{X, Y} || X <- Ints, Y <- Ints, X + Y == 2020]).

problem_2() ->
    Lines = readlines("src/aoc2020_day1_input.txt"),
    ParseFun = fun(Line) -> 
                       {I, _} = string:to_integer(Line),
                       I
               end,
    FilterFun = fun(Val) -> is_number(Val) end,
    Ints = lists:filter(FilterFun, lists:map(ParseFun, Lines)),
    hd([{X, Y, Z} || X <- Ints, Y <- Ints, Z <- Ints, X + Y + Z == 2020]).

readlines(FileName) ->
    {ok, Data} = file:read_file(FileName),
    binary:split(Data, [<<"\n">>], [global]).

