-module(pickler).

-include("cargo.hrl").

-export([
    to_pickle/1,
    from_pickle/1
]).

-on_load(init/0).

init() ->
    ?load_nif_from_crate(pickler, 0).

-spec to_pickle(term()) -> binary().
to_pickle(_) ->
    erlang:nif_error(nif_not_loaded).

-spec from_pickle(binary()) -> term().
from_pickle(_) ->
    erlang:nif_error(nif_not_loaded).
