{-# LANGUAGE CPP #-}
{-# LANGUAGE NoRebindableSyntax #-}
{-# OPTIONS_GHC -fno-warn-missing-import-lists #-}
module Paths_acronym (
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
version = Version [1,7,0,11] []
bindir, libdir, dynlibdir, datadir, libexecdir, sysconfdir :: FilePath

bindir     = "/home/myoungjin/exercism/haskell/acronym/.stack-work/install/x86_64-linux/510c11f8b0c881a1fe48bb76a673f5228c2caca16333a0f66e103b56986d5ed0/8.10.7/bin"
libdir     = "/home/myoungjin/exercism/haskell/acronym/.stack-work/install/x86_64-linux/510c11f8b0c881a1fe48bb76a673f5228c2caca16333a0f66e103b56986d5ed0/8.10.7/lib/x86_64-linux-ghc-8.10.7/acronym-1.7.0.11-IbfAfjNQqOH7CzaqSlZz46"
dynlibdir  = "/home/myoungjin/exercism/haskell/acronym/.stack-work/install/x86_64-linux/510c11f8b0c881a1fe48bb76a673f5228c2caca16333a0f66e103b56986d5ed0/8.10.7/lib/x86_64-linux-ghc-8.10.7"
datadir    = "/home/myoungjin/exercism/haskell/acronym/.stack-work/install/x86_64-linux/510c11f8b0c881a1fe48bb76a673f5228c2caca16333a0f66e103b56986d5ed0/8.10.7/share/x86_64-linux-ghc-8.10.7/acronym-1.7.0.11"
libexecdir = "/home/myoungjin/exercism/haskell/acronym/.stack-work/install/x86_64-linux/510c11f8b0c881a1fe48bb76a673f5228c2caca16333a0f66e103b56986d5ed0/8.10.7/libexec/x86_64-linux-ghc-8.10.7/acronym-1.7.0.11"
sysconfdir = "/home/myoungjin/exercism/haskell/acronym/.stack-work/install/x86_64-linux/510c11f8b0c881a1fe48bb76a673f5228c2caca16333a0f66e103b56986d5ed0/8.10.7/etc"

getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir, getSysconfDir :: IO FilePath
getBinDir = catchIO (getEnv "acronym_bindir") (\_ -> return bindir)
getLibDir = catchIO (getEnv "acronym_libdir") (\_ -> return libdir)
getDynLibDir = catchIO (getEnv "acronym_dynlibdir") (\_ -> return dynlibdir)
getDataDir = catchIO (getEnv "acronym_datadir") (\_ -> return datadir)
getLibexecDir = catchIO (getEnv "acronym_libexecdir") (\_ -> return libexecdir)
getSysconfDir = catchIO (getEnv "acronym_sysconfdir") (\_ -> return sysconfdir)

getDataFileName :: FilePath -> IO FilePath
getDataFileName name = do
  dir <- getDataDir
  return (dir ++ "/" ++ name)
