use nom::{IResult, Parser};

pub fn map_err<I, O, E1, E2, F, G>(mut first: F, second: G) -> impl FnMut(I) -> IResult<I, O, E2>
where
    F: Parser<I, O, E1>,
    G: Fn(E1) -> E2,
{
    move |input: I| match first.parse(input) {
        Ok(t) => Ok(t),
        Err(nom::Err::Incomplete(n)) => Err(nom::Err::Incomplete(n)),
        Err(nom::Err::Error(e)) => Err(nom::Err::Error(second(e))),
        Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(second(e))),
    }
}
