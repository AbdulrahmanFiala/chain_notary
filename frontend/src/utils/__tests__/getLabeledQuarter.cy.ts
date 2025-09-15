// src/__tests__/getLabeledQuarter.cy.ts

import { expect } from "chai";
import getLabeledQuarter from "../getLabeledQuarter";

describe('getLabeledQuarter', () => {
  describe('Valid quarter numbers (1-4)', () => {
    it('should return "1st" for quarter 1', () => {
      expect(getLabeledQuarter(1)).to.equal('1st');
    });

    it('should return "2nd" for quarter 2', () => {
      expect(getLabeledQuarter(2)).to.equal('2nd');
    });

    it('should return "3rd" for quarter 3', () => {
      expect(getLabeledQuarter(3)).to.equal('3rd');
    });

    it('should return "4th" for quarter 4', () => {
      expect(getLabeledQuarter(4)).to.equal('4th');
    });
  });

  describe('Invalid quarter numbers (outside 1-4 range)', () => {
    it('should return the original number for quarter 0', () => {
      expect(getLabeledQuarter(0)).to.equal(0);
    });

    it('should return the original number for quarter 5', () => {
      expect(getLabeledQuarter(5)).to.equal(5);
    });

    it('should return the original number for negative quarters', () => {
      expect(getLabeledQuarter(-1)).to.equal(-1);
      expect(getLabeledQuarter(-5)).to.equal(-5);
    });

    it('should return the original number for large positive quarters', () => {
      expect(getLabeledQuarter(10)).to.equal(10);
      expect(getLabeledQuarter(100)).to.equal(100);
    });
  });

  describe('Edge cases and type handling', () => {
    it('should handle decimal numbers by returning the original value', () => {
      expect(getLabeledQuarter(1.5)).to.equal(1.5);
      expect(getLabeledQuarter(3.7)).to.equal(3.7);
    });

    it('should handle zero', () => {
      expect(getLabeledQuarter(0)).to.equal(0);
    });

    it('should handle very large numbers', () => {
      expect(getLabeledQuarter(999999)).to.equal(999999);
    });

    it('should handle very small negative numbers', () => {
      expect(getLabeledQuarter(-999999)).to.equal(-999999);
    });
  });

  describe('Boundary testing', () => {
    it('should handle numbers just below valid range', () => {
      expect(getLabeledQuarter(0.9)).to.equal(0.9);
    });

    it('should handle numbers just above valid range', () => {
      expect(getLabeledQuarter(4.1)).to.equal(4.1);
    });

    it('should correctly identify the boundary values', () => {
      // Test exact boundaries
      expect(getLabeledQuarter(1)).to.equal('1st');
      expect(getLabeledQuarter(4)).to.equal('4th');
      
      // Test just outside boundaries
      expect(getLabeledQuarter(0.999)).to.equal(0.999);
      expect(getLabeledQuarter(4.001)).to.equal(4.001);
    });
  });

  describe('Integration scenarios', () => {
    it('should work correctly when called multiple times', () => {
      const quarters = [1, 2, 3, 4];
      const expected = ['1st', '2nd', '3rd', '4th'];
      
      quarters.forEach((quarter, index) => {
        expect(getLabeledQuarter(quarter)).to.equal(expected[index]);
      });
    });

    it('should handle mixed valid and invalid inputs', () => {
      const inputs = [0, 1, 2, 3, 4, 5];
      const expected = [0, '1st', '2nd', '3rd', '4th', 5];
      
      inputs.forEach((input, index) => {
        expect(getLabeledQuarter(input)).to.equal(expected[index]);
      });
    });
  });
});

// Alternative test structure using parameterized testing
describe('getLabeledQuarter - Parameterized Tests', () => {
  const validQuarters = [
    { input: 1, expected: '1st' },
    { input: 2, expected: '2nd' },
    { input: 3, expected: '3rd' },
    { input: 4, expected: '4th' },
  ];

  const invalidQuarters = [
    { input: 0, expected: 0 },
    { input: 5, expected: 5 },
    { input: -1, expected: -1 },
    { input: 10, expected: 10 },
    { input: 1.5, expected: 1.5 },
  ];

  validQuarters.forEach(({ input, expected }) => {
    it(`should return "${expected}" for input ${input}`, () => {
      expect(getLabeledQuarter(input)).to.equal(expected);
    });
  });

  invalidQuarters.forEach(({ input, expected }) => {
    it(`should return ${expected} for invalid input ${input}`, () => {
      expect(getLabeledQuarter(input)).to.equal(expected);
    });
  });
});