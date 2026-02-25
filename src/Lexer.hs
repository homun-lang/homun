module Lexer
  ( Token(..)
  , TokenKind(..)
  , lexHomun
  , tokenPos
  ) where

import Data.Char (isAlpha, isAlphaNum, isDigit, isSpace)
import Data.List (isPrefixOf)

-- | Source position for error reporting
data Pos = Pos { posLine :: Int, posCol :: Int }
  deriving (Show, Eq)

-- | A token with its source position
data Token = Token
  { tokenKind :: TokenKind
  , tokenPos  :: Pos
  } deriving (Show)

data TokenKind
  -- Literals
  = TInt    Integer
  | TFloat  Double
  | TBool   Bool
  | TString String
  | TNone
  -- Identifiers & Keywords
  | TIdent  String
  | TUse
  | TStruct
  | TEnum
  | TFor
  | TIn
  | TWhile
  | TDo
  | TIf
  | TElse
  | TMatch
  | TBreak
  | TContinue
  | TAnd
  | TOr
  | TNot
  | TAs
  | TRec
  -- Operators
  | TAssign
  | TArrow
  | TFatArrow
  | TPipe
  | TDot
  | TPlus
  | TMinus
  | TStar
  | TSlash
  | TPercent
  | TEq
  | TNeq
  | TLt
  | TGt
  | TLe
  | TGe
  | TColon
  | TComma
  | TSemi
  | TUnderscore
  | TAt
  -- Delimiters
  | TLParen
  | TRParen
  | TLBrace
  | TRBrace
  | TLBracket
  | TRBracket
  | TEOF
  deriving (Show, Eq)

lexHomun :: String -> Either String [Token]
lexHomun src = go src (Pos 1 1)
  where
    go [] pos = Right [Token TEOF pos]
    go s@(c:cs) pos
      | isSpace c
      = go cs (advancePos c pos)
      | "//" `isPrefixOf` s
      = go (dropWhile (/= '\n') s) pos
      | "/*" `isPrefixOf` s
      = case skipBlockComment (drop 2 s) (Pos (posLine pos) (posCol pos + 2)) of
          Left err           -> Left err
          Right (rest, pos') -> go rest pos'
      | c == '"'
      = case lexString cs (advancePos c pos) of
          Left err                -> Left err
          Right (str, rest, pos') ->
            fmap (Token (TString str) pos :) (go rest pos')
      | isDigit c
      = let (raw, rest) = span (\x -> isDigit x || x == '.') s
            tok = if '.' `elem` raw
                    then TFloat (read raw)
                    else TInt   (read raw)
        in fmap (Token tok pos :) (go rest (advancePosN (length raw) pos))
      | isAlpha c
      = let (word, rest) = span (\x -> isAlphaNum x || x == '_') s
        in fmap (Token (keyword word) pos :) (go rest (advancePosN (length word) pos))
      | ":=" `isPrefixOf` s = emit TAssign   2 s pos
      | "->" `isPrefixOf` s = emit TArrow    2 s pos
      | "=>" `isPrefixOf` s = emit TFatArrow 2 s pos
      | "==" `isPrefixOf` s = emit TEq       2 s pos
      | "!=" `isPrefixOf` s = emit TNeq      2 s pos
      | "<=" `isPrefixOf` s = emit TLe       2 s pos
      | ">=" `isPrefixOf` s = emit TGe       2 s pos
      | c == '|'  = emit TPipe       1 s pos
      | c == '.'  = emit TDot        1 s pos
      | c == '+'  = emit TPlus       1 s pos
      | c == '-'  = emit TMinus      1 s pos
      | c == '*'  = emit TStar       1 s pos
      | c == '/'  = emit TSlash      1 s pos
      | c == '%'  = emit TPercent    1 s pos
      | c == '<'  = emit TLt         1 s pos
      | c == '>'  = emit TGt         1 s pos
      | c == ':'  = emit TColon      1 s pos
      | c == ','  = emit TComma      1 s pos
      | c == ';'  = emit TSemi       1 s pos
      | c == '_'  = emit TUnderscore 1 s pos
      | c == '@'  = emit TAt         1 s pos
      | c == '('  = emit TLParen     1 s pos
      | c == ')'  = emit TRParen     1 s pos
      | c == '{'  = emit TLBrace     1 s pos
      | c == '}'  = emit TRBrace     1 s pos
      | c == '['  = emit TLBracket   1 s pos
      | c == ']'  = emit TRBracket   1 s pos
      | otherwise
      = Left $ "Unexpected character '" ++ [c]
             ++ "' at line " ++ show (posLine pos)
             ++ ", col " ++ show (posCol pos)
    emit tok n s pos =
      fmap (Token tok pos :) (go (drop n s) (advancePosN n pos))
    advancePos '\n' (Pos l _) = Pos (l + 1) 1
    advancePos _    (Pos l c) = Pos l (c + 1)
    advancePosN 0 pos         = pos
    advancePosN n (Pos l c)   = Pos l (c + n)

skipBlockComment :: String -> Pos -> Either String (String, Pos)
skipBlockComment [] _ = Left "Unterminated block comment"
skipBlockComment ('*':'/':rest) pos = Right (rest, pos)
skipBlockComment (c:cs) pos =
  let pos' = if c == '\n'
               then Pos (posLine pos + 1) 1
               else Pos (posLine pos) (posCol pos + 1)
  in skipBlockComment cs pos'

lexString :: String -> Pos -> Either String (String, String, Pos)
lexString [] _ = Left "Unterminated string literal"
lexString ('"' : rest) pos = Right ("", rest, pos)
lexString ('\\' : c : rest) pos =
  let escaped = case c of { 'n' -> '\n'; 't' -> '\t'; '\\' -> '\\'; '"' -> '"'; _ -> c }
      pos' = Pos (posLine pos) (posCol pos + 2)
  in case lexString rest pos' of
       Left e          -> Left e
       Right (s, r, p) -> Right (escaped : s, r, p)
lexString (c : rest) pos =
  let pos' = Pos (posLine pos) (posCol pos + 1)
  in case lexString rest pos' of
       Left e          -> Left e
       Right (s, r, p) -> Right (c : s, r, p)

keyword :: String -> TokenKind
keyword s = case s of
  "use"      -> TUse
  "struct"   -> TStruct
  "enum"     -> TEnum
  "for"      -> TFor
  "in"       -> TIn
  "while"    -> TWhile
  "do"       -> TDo
  "if"       -> TIf
  "else"     -> TElse
  "match"    -> TMatch
  "break"    -> TBreak
  "continue" -> TContinue
  "and"      -> TAnd
  "or"       -> TOr
  "not"      -> TNot
  "as"       -> TAs
  "rec"      -> TRec
  "true"     -> TBool True
  "false"    -> TBool False
  "none"     -> TNone
  _          -> TIdent s