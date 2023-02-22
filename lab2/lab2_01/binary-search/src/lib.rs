// iterative version
pub fn find_iterative(array: &[i32], key: i32) -> Option<usize> {
    let array = array;
    let key = key;
    let array_length = array.len();

    // Array vuoto
    if array_length == 0 { return None }

    let mut low = 0;
    let mut high = array_length - 1;

    // Chiave più piccola del valore più piccolo del vettore
    if key < array[low] { return None }

    while low <= high {
        let mid = low + (high - low)/2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    return None;
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // Array vuoto
    if array.len() == 0 { return None }
    let low = 0;
    let high = array.len() - 1;
    // Chiave più piccola del valore più piccolo del vettore
    if key < array[low] { return None }

    if key == array[low]  { return Some(low) }
    if key == array[high]  { return Some(high) }

    let mid = low + (high - low)/2;
    if key == array[mid] {
        return Some(mid);
    } else if key < array[mid] {
        find(array.get(0..mid+1).unwrap(), key);
    } else {
        find(array.get(mid+1..high).unwrap(), key);
    }
    return None;
}