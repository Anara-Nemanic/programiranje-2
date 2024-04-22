// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  (int -> int) -> int -> int
//  ('a -> 'b) -> 'a -> 'b
//  ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, da funkcija ne spremeni elementov seznama)
//  map_and_mutate: ('a -> 'b) -> 'a list -> 'b list // Definirajte funkcijo tako, da lahko zaprtje spremeni elemente seznama
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi zaprtje n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega
