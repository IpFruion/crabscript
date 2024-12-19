import XCTest
import SwiftTreeSitter
import TreeSitterCrabscript

final class TreeSitterCrabscriptTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_crabscript())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Crabscript grammar")
    }
}
