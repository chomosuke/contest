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

let find_z str i j =
  let rec loop str i j len =
    if String.(length str <= j || length str <= i || get str i <> get str j)
    then len
    else loop str (i + 1) (j + 1) (len + 1)
  in
  loop str i j 0

let z str =
  let rec loop str i l r z =
    if i < String.length str then (
      if i < r then (
        let zk = z.(i - l) in
        if zk < r - i then (
          (* prefix starting at k does not extend beyond r *)
          z.(i) <- zk;
          loop str (i + 1) l r z)
        else
          (* prefix starting at k might extend beyond r *)
          let zi = r - i + find_z str r (r - i) in
          z.(i) <- zi;
          loop str (i + 1) i (i + zi) z)
      else
        let zi = find_z str 0 i in
        z.(i) <- zi;
        loop str (i + 1) i (i + zi) z)
    else z
  in
  loop str 1 0 0 (Array.make (String.length str) (String.length str))

let z_arr = ref (Array.make 0 0)

let prefix_len str i =
  if i = 0 then String.length str
  else (
    if Array.length !z_arr = 0 then z_arr := z str;
    !z_arr.(i))

(*
We want to find the smallest prefix repeated infinite time.
*)
let find_cut_off str =
  let rec loop str m i =
    if i < String.length str then
      (* m is length, i is last char in current prefix *)
      String.(
        let prev = get str (i mod m) in
        let curr = get str i in
        if curr < prev then (* new best m *) loop str (i + 1) (i + 1)
        else if curr > prev then (* can never get better *) m
        else if i = (m * 2) - 1 then loop str (i + 1) (i + 1)
        else
          let a_len = (i + 1 - m) in
          let d = prefix_len str a_len in
          let a = get str d in
          let b = get str ((d + a_len) mod String.length str) in
          if a <= b then loop str (i + 1) (i + 1) else loop str m (i + 1))
    else m
  in
  loop str 1 1

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
