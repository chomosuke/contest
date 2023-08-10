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

let () =
  let rec loop i n =
    if i mod 3 <> 0 && i mod 5 <> 0 then print_int i;
    if i mod 3 = 0 then print_string "Fizz";
    if i mod 5 = 0 then print_string "Buzz";
    print_newline ();
    if i < n then loop (i + 1) n
  in
  loop 1 100
