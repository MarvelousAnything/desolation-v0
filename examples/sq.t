fun sq(n) {
    return .n * .n
}


fun init() {
    var i
    sprint("Table of squares:\n")
    i : 1
    loop {
        until .i >= 10
        iprint(.i)
        sprint(" squared equals ")
        iprint(sq(.i))
        nl()
        i : .i + 1
    }
}