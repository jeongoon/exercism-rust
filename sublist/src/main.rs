#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


// Any imperative lanugae version
pub fn sublist<T: PartialEq + Clone>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // okay. I could declare helper function inside of a function

    fn sublist_helper<T: PartialEq + Clone>
        (_short_list: &[T], _long_list: &[T])
         -> Comparison
    {
        let (mut short_list_cursor, mut long_list_cursor) = (0,0);
        let mut long_list_start = 0;
        let mut matched_count = 0;
        let (short_list_len, long_list_len) =
            ( _short_list.len(), _long_list.len() );

        while short_list_cursor < short_list_len &&
            long_list_cursor < long_list_len
        {
            if _short_list[short_list_cursor] == _long_list[long_list_cursor] {
                matched_count     += 1;
                short_list_cursor += 1;
                long_list_cursor  += 1;
            }
            else {
                matched_count     = 0;
                short_list_cursor = 0;
                long_list_start  += 1;
                long_list_cursor  = long_list_start;
            }
        }

        if short_list_cursor == long_list_cursor {
            if short_list_len == long_list_len {
                Comparison::Equal
            } else {
                Comparison::Sublist
            }
        } else if short_list_len == matched_count {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
}

    if _first_list.len() < _second_list.len() {
        return sublist_helper( _first_list, _second_list );

    } else {
        let conc = sublist_helper( _second_list, _first_list );
        match conc {
            Comparison::Sublist => Comparison::Superlist,
            _ => conc,
        }
    }
}

fn main() {
    assert_eq!( sublist( &[1,1,2], &[1,1,1,2]   ), Comparison::Sublist );
    assert_eq!( sublist( &[1,2],     &[2,3,4] ), Comparison::Unequal   );
    assert_eq!( sublist( &[4,5], &[1,2,3,4,5] ), Comparison::Sublist );
    assert_eq!( sublist( &[3,6,9], &[3,6,9]   ), Comparison::Equal );
}
