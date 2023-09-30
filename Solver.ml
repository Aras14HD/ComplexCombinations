let memo_rec f =
  let h = Hashtbl.create 16 in
  let rec g x =
    try Hashtbl.find h x
    with Not_found ->
      let y = f g x in
      Hashtbl.add h x y;
      y
  in
  g

let rec sum l = match l with
  | [] -> 0;
  | h::t -> h + (sum t)

let rec upto i l = match l with 
  | [] -> []
  | h::t -> if i == 0 then [] else h::(upto (i-1) t)

let rec fromon i l = match l with
  | [] -> []
  | _::t -> if i == 0 then t else fromon (i-1) t

let comb =
  let inner self arr =
    match arr with
    | [] -> 0
    | [1] -> 1
    | _ -> sum (List.mapi (fun i x -> if x <= List.nth (arr@[0]) (i+1) then 0 else
      (x - List.nth (arr@[0]) (i+1)) * 
      self (List.filter (fun x -> x>0) ((upto i arr) @ (x-1)::(fromon i arr)))) arr)
  in
  memo_rec inner

let () = 
  let l = List.map (fun x -> int_of_string x) (List.tl (Array.to_list Sys.argv)) in
  let t = Sys.time() in
  let res = comb l in
  let len = Sys.time() -. t in
  Printf.printf "Took: %f ms\nGot: %d" (len *. 1000.) res
