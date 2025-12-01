-module(hello).
-export([start/0, factorial/1]).

-record(person, {name, age}).

start() ->
    Person = #person{name = "Alice", age = 30},
    io:format("Hello, ~s!~n", [Person#person.name]),
    Result = factorial(5),
    io:format("5! = ~p~n", [Result]).

factorial(0) -> 1;
factorial(N) when N > 0 ->
    N * factorial(N - 1).
