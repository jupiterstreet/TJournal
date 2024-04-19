# T JOURNAL
CURRENT FLOW:

1. > parse args
2. > check/create dir.

3. > tj open 
(noArgs: creates new document for today)
(1arg: date ??/??/????, opens first if exists, creates if no exists) 
(2arg: date ??/??/????, specific #, opens if exists, creates if no exists)

4. > tj list 
(noArgs: lists entries for the current week, day by day)
- tj list -t (query 'tag')
- tj list -m (??/????)
- tj list bef (??/??/????)
- tj list aft (??/??/????)

5. > tj cal 
(noArgs: display calendar view for current month)
(month ??/????, display calendar view for given month)
-----------------------------------------------------
TODO/GENERAL IDEAS
> bigevent 
tags with !BIGEVENT get displayed in calendar 
> more search functionality with open:
-> adding a weekday to query just opens that day for current week