
fn smallest_factor(number)
    if number mod 2 == 0
        return 2
    else if number mod 3 == 0
        return 3
    else if number mod 5 == 0
        return 5

    let factor = 7
    while factor < number / 5
        if number mod factor == 0
            return factor
        else if number mod (factor + 3) == 0
            return factor + 3
        else if number mod (factor + 6) == 0
            return factor + 6
        else if number mod (factor + 10) == 0
            return factor + 10
        else if number mod (factor + 12) == 0
            return factor + 12
        else if number mod (factor + 16) == 0
            return factor + 16
        else if number mod (factor + 22) == 0
            return factor + 22
        else if number mod (factor + 24) == 0
            return factor + 24
        else if number mod (factor + 30) == 0
            return factor + 30
        else
            factor = factor + 30

    return number

return smallest_factor(8069 * 4373 * 4519)
