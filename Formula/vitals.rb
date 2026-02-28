# typed: false
# frozen_string_literal: true

# Homebrew formula for vitals — universal dev environment doctor
# To use: brew tap onuroluc/tap && brew install vitals
#
# This file is a template. The release CI auto-updates the tap repo
# with correct versions and SHA256 sums on each tagged release.
class Vitals < Formula
  desc "Universal development environment doctor — auto-detects and diagnoses project health"
  homepage "https://github.com/onuroluc/vitals"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/onuroluc/vitals/releases/download/v#{version}/vitals-darwin-arm64.tar.gz"
      sha256 "PLACEHOLDER"
    else
      url "https://github.com/onuroluc/vitals/releases/download/v#{version}/vitals-darwin-amd64.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/onuroluc/vitals/releases/download/v#{version}/vitals-linux-arm64.tar.gz"
      sha256 "PLACEHOLDER"
    else
      url "https://github.com/onuroluc/vitals/releases/download/v#{version}/vitals-linux-amd64.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  def install
    bin.install "vitals"
  end

  test do
    assert_match "vitals", shell_output("#{bin}/vitals --help")
  end
end
