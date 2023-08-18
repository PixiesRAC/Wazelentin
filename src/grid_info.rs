use grid::Error;

pub struct GridInfo
{
    pub start_pos : (usize, usize),
    pub row_max : usize,
    pub column_max : usize,
}
mod grid
{
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Error
    {
        WrongOrMissingRowColumnMax = 0,
        ErrorCastForRowColumnMax,
        WrongOrMissingStartPos,
        ErrorCastForStartPos,
        IncoherenceBetweenStartPosAndMaxBound,
        MisingExit,
        TooManyExit,
        IncoherenceGridSize, 
        UnothorizedValue
    }
}
impl GridInfo
{
    const CASE_OPEN: char = 'O';
    const CASE_CLOSE: char = 'E';
    const CASE_WIN: char = 'X';

    fn check_integrity(grid_row : &String) -> Result<(), Error>
    {
        let mut grid_lines = grid_row.lines();
        if let Some(row_and_column_max_line) = grid_lines.next() {
            let field_iterator: Vec<_> = row_and_column_max_line.split(' ').collect();
            if field_iterator.len() == 2 {
                if let Err(_) = field_iterator[0].parse::<usize>()
                {
                    return Err(Error::ErrorCastForRowColumnMax);
                }
                if let Err(_) = field_iterator[1].parse::<usize>()
                {
                    return Err(Error::ErrorCastForRowColumnMax);
                }
            }
            else {
                return Err(Error::WrongOrMissingRowColumnMax);
            }
        }
        if let Some(start_pos) = grid_lines.next() {
            let field_iterator: Vec<_> = start_pos.split(' ').collect();
            if field_iterator.len() == 2 {
                if let Err(_) = field_iterator[0].parse::<usize>()
                {
                    return Err(Error::ErrorCastForStartPos);
                }
                if let Err(_) = field_iterator[1].parse::<usize>()
                {
                    return Err(Error::ErrorCastForStartPos);
                }
            }
            else {
                return Err(Error::WrongOrMissingStartPos);
            }
        }
        Ok(())
    }

    fn check_logic(&self, lines : &mut std::str::Lines) -> Result<(), Error> 
    {
       if self.start_pos.0 > self.row_max
        {
            return Err(Error::IncoherenceBetweenStartPosAndMaxBound);
        }
       else if self.column_max < self.start_pos.1
        {
            return Err(Error::IncoherenceBetweenStartPosAndMaxBound);
        }
        let mut row : usize = 0;
        let mut nbExit = 0;
        for line in lines
        {
            if line.chars().any(|c| c != GridInfo::CASE_WIN && c != GridInfo::CASE_OPEN && c != GridInfo::CASE_CLOSE)
            {
                return Err(Error::UnothorizedValue);
            }
            nbExit += line.chars().filter(|&c| c == GridInfo::CASE_WIN).count();
            if (line.len() -1) != self.column_max // start by 0 even in the grid file
            {
                return Err(Error::IncoherenceGridSize)
            }
            row += 1
        }
        if (row -1) != self.row_max
        {
            return Err(Error::IncoherenceGridSize);
        }
        if nbExit == 0
        {
            return Err(Error::MisingExit);
        }
        else if nbExit > 1
        {
            return Err(Error::TooManyExit);
        }
        Ok(())
    }

    pub fn new(grid_row : &String) -> Result<(GridInfo), Error>
    {
        let mut grid_info = GridInfo {
            start_pos: (usize::MAX, usize::MAX),
            row_max: usize::MAX,
            column_max: usize::MAX,
        };

        GridInfo::check_integrity(grid_row)?;
        let mut grid_lines: std::str::Lines<'_> = grid_row.lines(); // why must it be mut ?
        if let Some(row_and_column_max_line) = grid_lines.next() {
            let field_iterator: Vec<_> = row_and_column_max_line.split(' ').collect();
            if let Ok(row) = field_iterator[0].parse::<usize>()
            {
                grid_info.row_max = row;
            }
            if let Ok(column) = field_iterator[1].parse::<usize>()
            {
                grid_info.column_max = column;
            } 
        }

        if let Some(start_pos) = grid_lines.next() {
            let field_iterator: Vec<_> = start_pos.split(' ').collect();
            if let Ok(start_row) = field_iterator[0].parse::<usize>()
            {
                grid_info.start_pos.0 = start_row;
            }
            if let Ok(start_column) = field_iterator[1].parse::<usize>()
            {
                grid_info.start_pos.1 = start_column;
            } 
        }

        grid_info.check_logic(&mut grid_lines)?;

        Ok(grid_info)
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_should_return_grid_info_with_valid_value() {
        let test_input_grid_row = &String::from("2 6\n0 2\nOXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Ok(grid_info) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(2, grid_info.row_max);
            assert_eq!(6, grid_info.column_max);
            assert_eq!((0,2), grid_info.start_pos);
        }
        else {
            panic!("Expected OK, but got an Error");
        }
    }
    #[test]
    fn test_should_return_error_wrong_or_missing_for_row_with_missing_value() {
        let test_input_grid_row = &String::from("6\n0 2\nOXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::WrongOrMissingRowColumnMax, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_error_wrong_or_missing_for_row_with_addidional_value() {
        let test_input_grid_row = &String::from("6 6 6\n0 2\nOXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::WrongOrMissingRowColumnMax, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_error_cast_for_row_with_incorrect_value() {
        let test_input_grid_row = &String::from("6 r\n0 2\nOXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::ErrorCastForRowColumnMax, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_error_wrong_or_missing_for_start_pos_with_missing_value() {
        let test_input_grid_row = &String::from("6 2\n2\nOXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::WrongOrMissingStartPos, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_error_wrong_or_missing_for_start_pos_with_additional_value() {
        let test_input_grid_row = &String::from("6 2\n2 2 2\nOXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::WrongOrMissingStartPos, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_error_cast_for_start_pos_with_incorrect_value() {
        let test_input_grid_row = &String::from("6 2\n0 p\nOXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::ErrorCastForStartPos, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_error_cast_for_doing_shit() {
        let test_input_grid_row = &String::from("6 22OXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::ErrorCastForRowColumnMax, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_error_cast_for_doing_shit_again() {
        let test_input_grid_row = &String::from("6 2\n2O XOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::ErrorCastForStartPos, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_incoherence_error_for_start_pos_superior_to_row_or_column() {
        let test_input_grid_row = &String::from("6 2\n6 3\nXOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::IncoherenceBetweenStartPosAndMaxBound, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_incoherence_error_for_grid_which_does_not_respect_bound() {
        let test_input_grid_row = &String::from("6 2\n6 2\nXOEEEE\nOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::IncoherenceGridSize, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_incoherence_error_for_grid_which_does_not_respect_bound_again() {
        let test_input_grid_row = &String::from("2 6\n0 2\nOXOEEEE\nOOOOOEE\nEOOEEEE\nE\n");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::IncoherenceGridSize, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_missing_exit_error_for_grid_which_does_not_have_exit() {
        let test_input_grid_row = &String::from("2 6\n0 2\nOOOEEEE\nOOOOOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::MisingExit, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_multiple_exist_for_grid_which_have_more_than_one_exit() {
        let test_input_grid_row = &String::from("2 6\n0 2\nOOOEEEE\nOOXXOEE\nEOOEEEE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::TooManyExit, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
    #[test]
    fn test_should_return_error_if_an_unothorized_value_is_present(){
        let test_input_grid_row = &String::from("2 6\n0 2\nOOOAEEE\nOOOOOEE\nEOOEEXE");
        if let Err(grid_error) = GridInfo::new(test_input_grid_row)
        {
            assert_eq!(Error::UnothorizedValue, grid_error);
        }
        else {
            panic!("Expected an error, but got Ok");
        }
    }
}
