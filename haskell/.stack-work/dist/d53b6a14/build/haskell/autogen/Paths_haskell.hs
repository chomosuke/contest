{-# LANGUAGE CPP #-}
{-# LANGUAGE NoRebindableSyntax #-}
{-# OPTIONS_GHC -fno-warn-missing-import-lists #-}
{-# OPTIONS_GHC -Wno-missing-safe-haskell-mode #-}
module Paths_haskell (
    version,
    getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir,
    getDataFileName, getSysconfDir
  ) where

import qualified Control.Exception as Exception
import Data.Version (Version(..))
import System.Environment (getEnv)
import Prelude

#if defined(VERSION_base)

#if MIN_VERSION_base(4,0,0)
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#else
catchIO :: IO a -> (Exception.Exception -> IO a) -> IO a
#endif

#else
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#endif
catchIO = Exception.catch

version :: Version
version = Version [0,1,0,0] []
bindir, libdir, dynlibdir, datadir, libexecdir, sysconfdir :: FilePath

bindir     = "C:\\Users\\a1332\\Desktop\\Code\\contest\\haskell\\.stack-work\\install\\12e25cde\\bin"
libdir     = "C:\\Users\\a1332\\Desktop\\Code\\contest\\haskell\\.stack-work\\install\\12e25cde\\lib\\x86_64-windows-ghc-9.0.2\\haskell-0.1.0.0-23zl7XLBSXc6B9tH5kid4R-haskell"
dynlibdir  = "C:\\Users\\a1332\\Desktop\\Code\\contest\\haskell\\.stack-work\\install\\12e25cde\\lib\\x86_64-windows-ghc-9.0.2"
datadir    = "C:\\Users\\a1332\\Desktop\\Code\\contest\\haskell\\.stack-work\\install\\12e25cde\\share\\x86_64-windows-ghc-9.0.2\\haskell-0.1.0.0"
libexecdir = "C:\\Users\\a1332\\Desktop\\Code\\contest\\haskell\\.stack-work\\install\\12e25cde\\libexec\\x86_64-windows-ghc-9.0.2\\haskell-0.1.0.0"
sysconfdir = "C:\\Users\\a1332\\Desktop\\Code\\contest\\haskell\\.stack-work\\install\\12e25cde\\etc"

getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir, getSysconfDir :: IO FilePath
getBinDir = catchIO (getEnv "haskell_bindir") (\_ -> return bindir)
getLibDir = catchIO (getEnv "haskell_libdir") (\_ -> return libdir)
getDynLibDir = catchIO (getEnv "haskell_dynlibdir") (\_ -> return dynlibdir)
getDataDir = catchIO (getEnv "haskell_datadir") (\_ -> return datadir)
getLibexecDir = catchIO (getEnv "haskell_libexecdir") (\_ -> return libexecdir)
getSysconfDir = catchIO (getEnv "haskell_sysconfdir") (\_ -> return sysconfdir)

getDataFileName :: FilePath -> IO FilePath
getDataFileName name = do
  dir <- getDataDir
  return (dir ++ "\\" ++ name)
