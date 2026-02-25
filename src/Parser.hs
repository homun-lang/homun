-- | Recursive-descent parser for Homun.
module Parser
  ( parseHomun
  ) where

import Lexer (Token(..), TokenKind(..))
import AST
import Data.List (intercalate)

type ParseError = String

newtype Parser a = Parser
  { runParser :: [Token] -> Either ParseError (a, [Token]) }

instance Functor Parser where
  fmap f (Parser p) = Parser $ \ts ->
    case p ts of
      Left e          -> Left e
      Right (a, ts')  -> Right (f a, ts')

instance Applicative Parser where
  pure a = Parser $ \ts -> Right (a, ts)
  Parser pf <*> Parser pa = Parser $ \ts -> do
    (f, ts')  <- pf ts
    (a, ts'') <- pa ts'
    return (f a, ts'')

instance Monad Parser where
  return = pure
  Parser pa >>= f = Parser $ \ts -> do
    (a, ts') <- pa ts
    runParser (f a) ts'

peek :: Parser Token
peek = Parser $ \ts -> case ts of
  []    -> Left "Unexpected end of input"
  (t:_) -> Right (t, ts)

advance :: Parser Token
advance = Parser $ \ts -> case ts of
  []       -> Left "Unexpected end of input"
  (t:rest) -> Right (t, rest)

expect :: TokenKind -> Parser Token
expect k = do
  t <- advance
  if tokenKind t == k
    then return t
    else failP $ "Expected " ++ show k ++ " but got " ++ show (tokenKind t)

failP :: String -> Parser a
failP msg = Parser $ \_ -> Left msg

-- | Try a parser; if it fails restore the token stream
tryP :: Parser a -> Parser (Maybe a)
tryP (Parser p) = Parser $ \ts ->
  case p ts of
    Left _        -> Right (Nothing, ts)
    Right (a, ts') -> Right (Just a, ts')

check :: TokenKind -> Parser Bool
check k = do
  t <- peek
  return (tokenKind t == k)

consume :: TokenKind -> Parser Bool
consume k = do
  b <- check k
  if b then advance >> return True else return False

-- ─────────────────────────────────────────
-- Entry point
-- ─────────────────────────────────────────

parseHomun :: [Token] -> Either ParseError Program
parseHomun tokens = do
  (prog, _) <- runParser parseProgram tokens
  return prog

parseProgram :: Parser Program
parseProgram = do
  stmts <- parseMany parseTopStmt
  expect TEOF
  return stmts

parseMany :: Parser (Maybe a) -> Parser [a]
parseMany p = do
  mx <- p
  case mx of
    Nothing -> return []
    Just x  -> fmap (x:) (parseMany p)

-- ─────────────────────────────────────────
-- Top-level statements
-- ─────────────────────────────────────────

parseTopStmt :: Parser (Maybe Stmt)
parseTopStmt = do
  t <- peek
  case tokenKind t of
    TEOF    -> return Nothing
    TRBrace -> return Nothing
    TUse    -> fmap Just parseUse
    TIdent _ -> fmap Just parseTopBind
    _        -> return Nothing  -- unknown at top level stops parsing

parseUse :: Parser Stmt
parseUse = do
  expect TUse
  path <- parseModPath
  return (SUse path)

-- Parse  a::b::{X, Y}  style paths (simplified: just collect idents separated by ::)
parseModPath :: Parser [Name]
parseModPath = do
  t <- advance
  case tokenKind t of
    TIdent n -> do
      -- check for :: (two colons)
      b <- check TColon
      if b then do
        advance          -- first colon
        b2 <- check TColon
        if b2 then do
          advance        -- second colon
          rest <- parseModPath
          return (n : rest)
        else return [n]
      else return [n]
    _ -> failP "Expected module name"

-- | Top-level:   name := struct { ... }
--                name := enum   { ... }
--                name := expr
parseTopBind :: Parser Stmt
parseTopBind = do
  t <- advance   -- consume TIdent
  let name = case tokenKind t of
               TIdent n -> n
               _ -> "_"
  expect TAssign
  -- peek to decide what kind of RHS we have
  t2 <- peek
  case tokenKind t2 of
    TStruct -> do
      advance
      fields <- parseBraceFields
      return (SStructDef name fields)
    TEnum -> do
      advance
      variants <- parseBraceVariants
      return (SEnumDef name variants)
    _ -> do
      rhs <- parseExpr
      return (SBind name rhs)

parseBraceFields :: Parser [FieldDef]
parseBraceFields = do
  expect TLBrace
  fields <- parseFieldList
  expect TRBrace
  return fields

parseFieldList :: Parser [FieldDef]
parseFieldList = do
  t <- peek
  case tokenKind t of
    TRBrace -> return []
    TIdent n -> do
      advance
      expect TColon
      ty <- parseTypeExpr
      _ <- consume TComma
      rest <- parseFieldList
      return (FieldDef n (Just ty) : rest)
    _ -> return []

parseBraceVariants :: Parser [VariantDef]
parseBraceVariants = do
  expect TLBrace
  vs <- parseVariantList
  expect TRBrace
  return vs

parseVariantList :: Parser [VariantDef]
parseVariantList = do
  t <- peek
  case tokenKind t of
    TRBrace -> return []
    TIdent n -> do
      advance
      -- optional payload: Name(Type)
      b <- check TLParen
      payload <- if b then do
        advance
        ty <- parseTypeExpr
        expect TRParen
        return (Just ty)
      else return Nothing
      _ <- consume TComma
      rest <- parseVariantList
      return (VariantDef n payload : rest)
    _ -> return []

-- ─────────────────────────────────────────
-- Block-level statements (inside { })
-- ─────────────────────────────────────────

parseBlockStmts :: Parser [Stmt]
parseBlockStmts = do
  t <- peek
  case tokenKind t of
    TRBrace -> return []
    TEOF    -> return []
    TIdent _ -> do
      s <- parseBlockBind
      rest <- parseBlockStmts
      return (s : rest)
    _ -> do
      e <- parseExpr
      rest <- parseBlockStmts
      return (SExprStmt e : rest)

parseBlockBind :: Parser Stmt
parseBlockBind = do
  -- Try  name :=  first
  mBind <- tryP $ do
    t <- advance
    let name = case tokenKind t of TIdent n -> n; _ -> "_"
    expect TAssign
    return name
  case mBind of
    Just name -> do
      rhs <- parseExpr
      return (SBind name rhs)
    Nothing -> do
      e <- parseExpr
      return (SExprStmt e)

-- ─────────────────────────────────────────
-- Expressions — Pratt precedence climbing
-- ─────────────────────────────────────────

parseExpr :: Parser Expr
parseExpr = parsePipe

-- Pipe is the lowest-priority binary operator
parsePipe :: Parser Expr
parsePipe = do
  lhs <- parseOr
  go lhs
  where
    go lhs = do
      b <- check TPipe
      if b then do
        advance
        rhs <- parseCall  -- rhs of pipe is a call (not full expr, to avoid re-piping)
        go (EPipe lhs rhs)
      else return lhs

parseOr :: Parser Expr
parseOr = do
  lhs <- parseAnd
  b <- check TOr
  if b then do advance; rhs <- parseOr; return (EBinOp OpOr lhs rhs)
  else return lhs

parseAnd :: Parser Expr
parseAnd = do
  lhs <- parseNot
  b <- check TAnd
  if b then do advance; rhs <- parseAnd; return (EBinOp OpAnd lhs rhs)
  else return lhs

parseNot :: Parser Expr
parseNot = do
  b <- check TNot
  if b then do advance; e <- parseNot; return (EUnOp OpNot e)
  else parseCmp

parseCmp :: Parser Expr
parseCmp = do
  lhs <- parseAddSub
  t <- peek
  case tokenKind t of
    TEq  -> do advance; rhs <- parseAddSub; return (EBinOp OpEq  lhs rhs)
    TNeq -> do advance; rhs <- parseAddSub; return (EBinOp OpNeq lhs rhs)
    TLt  -> do advance; rhs <- parseAddSub; return (EBinOp OpLt  lhs rhs)
    TGt  -> do advance; rhs <- parseAddSub; return (EBinOp OpGt  lhs rhs)
    TLe  -> do advance; rhs <- parseAddSub; return (EBinOp OpLe  lhs rhs)
    TGe  -> do advance; rhs <- parseAddSub; return (EBinOp OpGe  lhs rhs)
    TIn  -> do advance; rhs <- parseAddSub; return (EBinOp OpIn  lhs rhs)
    TNot -> do
      advance
      expect TIn
      rhs <- parseAddSub
      return (EBinOp OpNotIn lhs rhs)
    _    -> return lhs

parseAddSub :: Parser Expr
parseAddSub = do
  lhs <- parseMulDiv
  go lhs
  where
    go lhs = do
      t <- peek
      case tokenKind t of
        TPlus  -> do advance; rhs <- parseMulDiv; go (EBinOp OpAdd lhs rhs)
        TMinus -> do advance; rhs <- parseMulDiv; go (EBinOp OpSub lhs rhs)
        _      -> return lhs

parseMulDiv :: Parser Expr
parseMulDiv = do
  lhs <- parseUnary
  go lhs
  where
    go lhs = do
      t <- peek
      case tokenKind t of
        TStar    -> do advance; rhs <- parseUnary; go (EBinOp OpMul lhs rhs)
        TSlash   -> do advance; rhs <- parseUnary; go (EBinOp OpDiv lhs rhs)
        TPercent -> do advance; rhs <- parseUnary; go (EBinOp OpMod lhs rhs)
        _        -> return lhs

parseUnary :: Parser Expr
parseUnary = do
  t <- peek
  case tokenKind t of
    TMinus -> do advance; e <- parseUnary; return (EUnOp OpNeg e)
    _      -> parsePostfix

-- Call, field access, index — highest priority
parsePostfix :: Parser Expr
parsePostfix = do
  base <- parseAtom
  go base
  where
    go e = do
      t <- peek
      case tokenKind t of
        TDot -> do
          advance
          t2 <- advance
          case tokenKind t2 of
            TIdent n -> do
              b <- check TLParen
              if b then do
                args <- parseArgList
                go (ECall (EField e n) args)
              else go (EField e n)
            _ -> failP "Expected field name after '.'"
        TLBracket -> do
          advance
          result <- parseSliceOrIndex
          expect TRBracket
          case result of
            Left  idx       -> go (EIndex e idx)
            Right (a, b, c) -> go (ESlice e a b c)
        TLParen -> do
          args <- parseArgList
          go (ECall e args)
        _ -> return e

-- | A call on the RHS of a pipe — just atom + postfix, no full expression
-- (prevents  a | f | g  from being parsed as  a | (f | g))

-- | Parse index or slice inside [ ]
-- Returns Left for plain index, Right for (start, end, step) slice
parseSliceOrIndex :: Parser (Either Expr (Maybe Expr, Maybe Expr, Maybe Expr))
parseSliceOrIndex = do
  t <- peek
  case tokenKind t of
    TColon -> do
      advance
      parseSliceRest Nothing
    _ -> do
      e <- parseExpr
      t2 <- peek
      case tokenKind t2 of
        TColon -> do advance; parseSliceRest (Just e)
        _      -> return (Left e)

parseSliceRest :: Maybe Expr -> Parser (Either Expr (Maybe Expr, Maybe Expr, Maybe Expr))
parseSliceRest start = do
  end <- parseOptSliceExpr
  step <- do
    b <- consume TColon
    if b then parseOptSliceExpr else return Nothing
  return (Right (start, end, step))

parseOptSliceExpr :: Parser (Maybe Expr)
parseOptSliceExpr = do
  t <- peek
  case tokenKind t of
    TRBracket -> return Nothing
    TColon    -> return Nothing
    _         -> fmap Just parseExpr

parseCall :: Parser Expr
parseCall = parsePostfix

-- ─────────────────────────────────────────
-- Atoms
-- ─────────────────────────────────────────

parseAtom :: Parser Expr
parseAtom = do
  t <- peek
  case tokenKind t of
    TInt n    -> advance >> return (EInt n)
    TFloat n  -> advance >> return (EFloat n)
    TBool b   -> advance >> return (EBool b)
    TString s -> advance >> return (EString s)
    TNone     -> advance >> return ENone

    -- Lambda OR parenthesised expression
    TLParen -> parseLambdaOrParen

    TAt         -> parseCollection
    TIf         -> parseIfExpr
    TMatch      -> parseMatchExpr
    TFor        -> parseForExpr
    TWhile      -> parseWhileExpr
    TBreak      -> parseBreakExpr
    TContinue   -> advance >> return EContinue
    TLBrace     -> parseInlineBlock

    TUnderscore -> advance >> return (EVar "_")

    TIdent n -> do
      advance
      -- PascalCase ident followed by { → struct literal
      b <- check TLBrace
      if b && isUpperFirst n
        then do
          fields <- parseStructLitFields
          return (EStruct (Just n) fields)
        else return (EVar n)

    _ -> failP $ "Unexpected token in expression: " ++ show (tokenKind t)

-- ─────────────────────────────────────────
-- Lambda vs parenthesised expression
-- ─────────────────────────────────────────

-- | Try to parse a lambda  (params) -> ...
-- Fall back to  (expr)  if that fails.
parseLambdaOrParen :: Parser Expr
parseLambdaOrParen = do
  mL <- tryP parseLambda
  case mL of
    Just l -> return l
    Nothing -> do
      expect TLParen
      e <- parseExpr
      -- Check for tuple: (e1, e2, ...) or just (e)
      t <- peek
      case tokenKind t of
        TComma -> do
          advance
          rest <- parseExprSep TRParen
          expect TRParen
          return (ETuple (e : rest))
        _ -> do
          expect TRParen
          return e

parseLambda :: Parser Expr
parseLambda = do
  expect TLParen
  params <- parseParams
  expect TRParen
  expect TArrow
  -- Optional return type annotation between -> and {
  -- Void marker: -> _  { ... }
  -- Explicit type: -> int { ... }
  -- Inferred: -> { ... }
  t <- peek
  (retTy, voidMark) <- case tokenKind t of
    TLBrace     -> return (Nothing, Nothing)
    TUnderscore -> do advance; return (Nothing, Just TVoid)
    _           -> do ty <- parseTypeExpr; return (Just ty, Nothing)
  stmts <- parseBlockStmts'
  let (ss, fe) = splitBlock stmts
  return (ELambda params retTy voidMark ss fe)

parseParams :: Parser [Param]
parseParams = do
  t <- peek
  case tokenKind t of
    TRParen -> return []
    _ -> do
      p    <- parseOneParam
      rest <- parseParamTail
      return (p : rest)

parseParamTail :: Parser [Param]
parseParamTail = do
  b <- consume TComma
  if b then do
    t <- peek
    case tokenKind t of
      TRParen -> return []
      _ -> do p <- parseOneParam; rest <- parseParamTail; return (p : rest)
  else return []

parseOneParam :: Parser Param
parseOneParam = do
  t <- advance
  case tokenKind t of
    TIdent n -> do
      b <- consume TColon
      if b then do ty <- parseTypeExpr; return (Param n (Just ty))
      else return (Param n Nothing)
    TUnderscore -> return (Param "_" Nothing)
    _ -> failP $ "Expected parameter name, got " ++ show (tokenKind t)

-- ─────────────────────────────────────────
-- Collections
-- ─────────────────────────────────────────

parseCollection :: Parser Expr
parseCollection = do
  expect TAt
  t <- peek
  case tokenKind t of
    TLBracket -> parseList
    TLBrace   -> parseDict
    TLParen   -> parseSet
    _ -> failP $ "Expected [, { or ( after @, got " ++ show (tokenKind t)

parseList :: Parser Expr
parseList = do
  expect TLBracket
  t <- peek
  case tokenKind t of
    TRBracket -> advance >> return (EList [])
    _ -> do
      items <- parseExprSep TRBracket
      expect TRBracket
      return (EList items)

parseDict :: Parser Expr
parseDict = do
  expect TLBrace
  t <- peek
  case tokenKind t of
    TRBrace -> advance >> return (EDict [])
    _ -> do
      pairs <- parseDictPairs
      expect TRBrace
      return (EDict pairs)

parseDictPairs :: Parser [(Expr, Expr)]
parseDictPairs = do
  k <- parseExpr
  expect TColon
  v <- parseExpr
  t <- peek
  case tokenKind t of
    TComma -> do advance; rest <- parseDictPairs; return ((k,v):rest)
    _      -> return [(k,v)]

parseSet :: Parser Expr
parseSet = do
  expect TLParen
  items <- parseExprSep TRParen
  expect TRParen
  return (ESet items)

parseExprSep :: TokenKind -> Parser [Expr]
parseExprSep stop = do
  t <- peek
  if tokenKind t == stop then return [] else do
    e <- parseExpr
    t2 <- peek
    case tokenKind t2 of
      TComma -> do advance; rest <- parseExprSep stop; return (e:rest)
      _      -> return [e]

parseStructLitFields :: Parser [(Name, Expr)]
parseStructLitFields = do
  expect TLBrace
  fields <- go
  expect TRBrace
  return fields
  where
    go = do
      t <- peek
      case tokenKind t of
        TRBrace -> return []
        TIdent n -> do
          advance
          expect TColon
          v <- parseExpr
          _ <- consume TComma
          rest <- go
          return ((n,v):rest)
        _ -> return []

-- ─────────────────────────────────────────
-- Control flow
-- ─────────────────────────────────────────

parseIfExpr :: Parser Expr
parseIfExpr = do
  expect TIf
  expect TLParen
  cond <- parseExpr
  expect TRParen
  expect TDo
  thenStmts <- parseBlockStmts'
  let (ts, te) = splitBlock thenStmts
  t <- peek
  elseClause <- case tokenKind t of
    TElse -> do
      advance
      elseStmts <- parseBlockStmts'
      let (es, ee) = splitBlock elseStmts
      return (Just (es, ee))
    _ -> return Nothing
  return (EIf cond ts te elseClause)

parseMatchExpr :: Parser Expr
parseMatchExpr = do
  expect TMatch
  scrut <- parseExpr
  expect TLBrace
  arms <- parseMatchArms
  expect TRBrace
  return (EMatch scrut arms)

parseMatchArms :: Parser [MatchArm]
parseMatchArms = do
  t <- peek
  case tokenKind t of
    TRBrace -> return []
    _ -> do
      arm  <- parseMatchArm
      rest <- parseMatchArms
      return (arm:rest)

parseMatchArm :: Parser MatchArm
parseMatchArm = do
  pat <- parsePat
  guard_ <- do
    b <- check TIf
    if b then do advance; fmap Just parseExpr
    else return Nothing
  expect TFatArrow
  body <- parseExpr
  return (MatchArm pat guard_ body)

parsePat :: Parser Pat
parsePat = do
  t <- peek
  case tokenKind t of
    TUnderscore -> advance >> return PWild
    TNone       -> advance >> return PNone
    TLParen     -> do
      advance
      pats <- parsePatList
      expect TRParen
      return (PTuple pats)
    TIdent n -> do
      advance
      b <- check TDot
      if b then do
        advance
        t2 <- advance
        case tokenKind t2 of
          TIdent v -> do
            b2 <- check TLParen
            if b2 then do
              advance
              p <- parsePat
              expect TRParen
              return (PEnum (n ++ "." ++ v) (Just p))
            else return (PEnum (n ++ "." ++ v) Nothing)
          _ -> failP "Expected variant name after '.'"
      else return (PVar n)
    TInt n   -> advance >> return (PLit (EInt n))
    TFloat n -> advance >> return (PLit (EFloat n))
    TBool b  -> advance >> return (PLit (EBool b))
    TString s -> advance >> return (PLit (EString s))
    _ -> do e <- parseAtom; return (PLit e)

parsePatList :: Parser [Pat]
parsePatList = do
  t <- peek
  case tokenKind t of
    TRParen -> return []
    _ -> do
      p <- parsePat
      t2 <- peek
      case tokenKind t2 of
        TComma -> do advance; rest <- parsePatList; return (p:rest)
        _      -> return [p]

parseForExpr :: Parser Expr
parseForExpr = do
  expect TFor
  t <- advance
  varName <- case tokenKind t of
    TIdent n    -> return n
    TUnderscore -> return "_"
    _ -> failP "Expected loop variable"
  expect TIn
  iter <- parseExpr
  expect TDo
  stmts <- parseBlockStmts'
  let (ss, fe) = splitBlock stmts
  return (EFor varName iter ss (Just fe))

parseWhileExpr :: Parser Expr
parseWhileExpr = do
  expect TWhile
  expect TLParen
  cond <- parseExpr
  expect TRParen
  expect TDo
  stmts <- parseBlockStmts'
  let (ss, fe) = splitBlock stmts
  return (EWhile cond ss (Just fe))

parseBreakExpr :: Parser Expr
parseBreakExpr = do
  expect TBreak
  b <- check TFatArrow
  if b then do advance; e <- parseExpr; return (EBreak (Just e))
  else return (EBreak Nothing)

-- | An inline block used as an expression: { stmts... }
parseInlineBlock :: Parser Expr
parseInlineBlock = do
  stmts <- parseBlockStmts'
  let (ss, fe) = splitBlock stmts
  -- represent as a match true { _ => ... } so codegen can handle it
  return (EMatch (EBool True) [MatchArm PWild Nothing (wrapBlock ss fe)])

-- Wrap a sequence of stmts + final expr as a single expr using EFor placeholder
wrapBlock :: [Stmt] -> Expr -> Expr
wrapBlock [] e = e
wrapBlock ss e = EFor "__block__" (EList []) ss (Just e)

-- ─────────────────────────────────────────
-- Block helpers
-- ─────────────────────────────────────────

-- | Parse { stmts }
parseBlockStmts' :: Parser [Stmt]
parseBlockStmts' = do
  expect TLBrace
  stmts <- parseBlockStmts
  expect TRBrace
  return stmts

-- | Split stmts into (all-but-last, last-as-expr)
splitBlock :: [Stmt] -> ([Stmt], Expr)
splitBlock []    = ([], EBool True)
splitBlock stmts =
  case last stmts of
    SExprStmt e -> (init stmts, e)
    other       -> (stmts, EBool True)

-- ─────────────────────────────────────────
-- Argument list
-- ─────────────────────────────────────────

parseArgList :: Parser [Expr]
parseArgList = do
  expect TLParen
  args <- parseExprSep TRParen
  expect TRParen
  return args

-- ─────────────────────────────────────────
-- Types
-- ─────────────────────────────────────────

parseTypeExpr :: Parser TypeExpr
parseTypeExpr = do
  t <- peek
  case tokenKind t of
    TAt -> do
      advance
      t2 <- peek
      case tokenKind t2 of
        TLBracket -> do advance; ty <- parseTypeExpr; expect TRBracket; return (TList ty)
        TLBrace   -> do
          advance; kty <- parseTypeExpr; expect TColon; vty <- parseTypeExpr; expect TRBrace
          return (TDict kty vty)
        TLParen -> do advance; ty <- parseTypeExpr; expect TRParen; return (TSet ty)
        _ -> return TInfer
    TUnderscore -> advance >> return TVoid
    TIdent n    -> advance >> return (TName n)
    _           -> return TInfer

-- ─────────────────────────────────────────
-- Utilities
-- ─────────────────────────────────────────

isUpperFirst :: String -> Bool
isUpperFirst (c:_) = c >= 'A' && c <= 'Z'
isUpperFirst []    = False