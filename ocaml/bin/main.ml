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

let is_prime n =
  let rec loop x n =
    if Int64.mul x x > n then true
    else if Int64.rem n x = Int64.zero then false
    else loop (Int64.succ x) n
  in
  loop (Int64.of_int 2) n

let () =
  let n = _next Int64.of_string in
  print_endline
    (if is_prime n then "1"
     else if Int64.rem n (Int64.of_int 2) = Int64.zero then "2"
     else if is_prime (Int64.sub n (Int64.of_int 2)) then "2"
     else "3")
