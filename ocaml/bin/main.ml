let remaining_tokens = ref []

let rec split_on_char sep str =
  try
    let i = String.index str sep in
    String.sub str 0 i
    :: split_on_char sep (String.sub str (i + 1) (String.length str - i - 1))
  with Not_found -> [ str ]

let rec _next parse =
  match !remaining_tokens with
  | hd :: tl ->
      remaining_tokens := tl;
      parse hd
  | [] ->
      remaining_tokens := split_on_char ' ' (read_line ());
      _next parse

let _next_line () =
  assert (!remaining_tokens = []);
  read_line ()

(*
You will simply cut the string once and repeat it many time. This is because:
You want to find "it got smaller point." and make it as early as poissble.

Find first letter that's equal or bigger than start

If bigger, that's the cut off point.

If equal, then maybe if we take that as cut off point, it will be smaller at a later point.

If it does get smaller at a later point. This will work. This is because something it has always been smaller so
starting later can't possibly be bigger.

If it doesn't get smaller at a later point, we restart from this index and keep trying to find the cut off point.

Becuase everything after the cut off point is smaller or equal to the start. You will not make the string
lexically smaller by cutting less than a cycle. Cause you could've done that the first time around (probably)
*)
let find_cut_off str =
  let viable str i =
    let this = String.get str i in
    let start = String.get str 0 in
    start < this
    || start = this
       &&
       let rec viable str i j =
         String.(
           if length str <= j then true
           else if get str i < get str j then true
           else if get str i > get str j then false
           else viable str (i + 1) (j + 1))
       in
       viable str 0 i
  in
  let rec loop str i =
    String.(if length str <= i || viable str i then i else loop str (i + 1))
  in
  loop str 1

let () =
  let k =
    _next (fun _ -> ());
    _next int_of_string
  in
  let str = _next_line () in
  let cut_off = find_cut_off str in
  let rec print_all k str =
    let len = String.length str in
    if k > len then (
      print_string str;
      print_all (k - len) str)
    else print_endline (String.sub str 0 k)
  in
  print_all k (String.sub str 0 cut_off)
