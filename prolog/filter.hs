import Data.List (intersect)

main 
    = interact 
    (
        unlines
        . map (("word(`" ++) . (++ "`)."))
        . filter (null . intersect " -")
        . lines
    )
