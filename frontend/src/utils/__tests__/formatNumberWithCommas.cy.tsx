// cypress/support/unit/numberFormatter.spec.ts

import { useState } from "react";
import { formatNumberWithCommas } from "../formatNumberWithCommas";
import { expect } from "chai";



describe('formatNumberWithCommas', () => {
  it('should format numbers greater than 999 with commas', () => {
    // Test the main requirement: 1000 -> 1,000
    expect(formatNumberWithCommas(1000)).to.equal('1,000');

    // Test other numbers > 999
    expect(formatNumberWithCommas(1234)).to.equal('1,234');
    expect(formatNumberWithCommas(12345)).to.equal('12,345');
    expect(formatNumberWithCommas(123456)).to.equal('123,456');
    expect(formatNumberWithCommas(1234567)).to.equal('1,234,567');
    expect(formatNumberWithCommas(12345678)).to.equal('12,345,678');
  });

  it('should not format numbers less than or equal to 999', () => {
    expect(formatNumberWithCommas(999)).to.equal('999');
    expect(formatNumberWithCommas(500)).to.equal('500');
    expect(formatNumberWithCommas(100)).to.equal('100');
    expect(formatNumberWithCommas(50)).to.equal('50');
    expect(formatNumberWithCommas(1)).to.equal('1');
    expect(formatNumberWithCommas(0)).to.equal('0');
  });

  it('should handle negative numbers correctly', () => {
    expect(formatNumberWithCommas(-1000)).to.equal('-1,000');
    expect(formatNumberWithCommas(-1234)).to.equal('-1,234');
    expect(formatNumberWithCommas(-999)).to.equal('-999');
    expect(formatNumberWithCommas(-500)).to.equal('-500');
  });

  it('should handle decimal numbers', () => {
    expect(formatNumberWithCommas(1000.5)).to.equal('1,000.5');
    expect(formatNumberWithCommas(1234.56)).to.equal('1,234.56');
    expect(formatNumberWithCommas(999.99)).to.equal('999.99');
  });

  it('should handle edge cases', () => {
    expect(formatNumberWithCommas(1001)).to.equal('1,001');
    expect(formatNumberWithCommas(10000)).to.equal('10,000');
    expect(formatNumberWithCommas(100000)).to.equal('100,000');
    expect(formatNumberWithCommas(1000000)).to.equal('1,000,000');
  });
});

describe('Number Formatter Component Integration', () => {
  it('should integrate with a React component using JSX', () => {
    const TestComponent: React.FC = () => {
      const numbers = [999, 1000, 1234, 12345];
      return (
        <div>
          {numbers.map(num => (
            <span key={num} data-cy={`number-${num}`}>
              {formatNumberWithCommas(num)}
            </span>
          ))}
        </div>
      );
    };
    // @ts-expect-error Cypress mount types not properly configured
    cy.mount(<TestComponent />);

    cy.get('[data-cy="number-999"]').should('contain.text', '999');
    cy.get('[data-cy="number-1000"]').should('contain.text', '1,000');
    cy.get('[data-cy="number-1234"]').should('contain.text', '1,234');
    cy.get('[data-cy="number-12345"]').should('contain.text', '12,345');
  });
  it('should work in a table format', () => {
    interface NumberRowProps {
      number: number;
    }

    const NumberRow: React.FC<NumberRowProps> = ({ number }) => (
      <tr data-cy={`row-${number}`}>
        <td>{number}</td>
        <td data-cy={`formatted-${number}`}>{formatNumberWithCommas(number)}</td>
      </tr>
    );

    const NumberTable: React.FC = () => {
      const numbers = [500, 1000, 5000, 10000, 50000];
      return (
        <table>
          <thead>
            <tr>
              <th>Original</th>
              <th>Formatted</th>
            </tr>
          </thead>
          <tbody>
            {numbers.map(num => (
              <NumberRow key={num} number={num} />
            ))}
          </tbody>
        </table>
      );
    };
    // @ts-expect-error Cypress mount types not properly configured
    cy.mount(<NumberTable />);

    cy.get('[data-cy="formatted-500"]').should('contain.text', '500');
    cy.get('[data-cy="formatted-1000"]').should('contain.text', '1,000');
    cy.get('[data-cy="formatted-5000"]').should('contain.text', '5,000');
    cy.get('[data-cy="formatted-10000"]').should('contain.text', '10,000');
    cy.get('[data-cy="formatted-50000"]').should('contain.text', '50,000');
  });

  it('should work with interactive components', () => {
    const InteractiveFormatter: React.FC = () => {
      const [inputValue, setInputValue] = useState<number | string>('1000');
      const displayValue = inputValue === '' ? 0 : Number(inputValue);

      return (
        <div>
          <input
            data-cy="number-input"
            type="number"
            value={inputValue}
            onChange={(e) => {
              const value = e.target.value;
              setInputValue(value === '' ? '' : Number(value));
            }} />
          <div data-cy="formatted-output">
            {formatNumberWithCommas(displayValue)}
          </div>
        </div>
      );
    };
    // @ts-expect-error Cypress mount types not properly configured
    cy.mount(<InteractiveFormatter />);

    // Test initial state
    cy.get('[data-cy="formatted-output"]').should('contain.text', '1,000');

    // Test changing the input
    cy.get('[data-cy="number-input"]').clear().type('5000');
    cy.get('[data-cy="formatted-output"]').should('contain.text', '5,000');

    cy.get('[data-cy="number-input"]').clear().type('999');
    cy.get('[data-cy="formatted-output"]').should('contain.text', '999');
  });
});
