<?php

declare(strict_types=1);

namespace Tests;

use PHPUnit\Framework\TestCase;

final class CalculatorTest extends TestCase
{
    public function testCalculator(): void
    {
        self::assertEquals(4, sum_numbers(2, 2));
        self::assertEquals(0, sum_numbers(2, -2));
        self::assertEquals(-4, sum_numbers(-2, -2));
    }
}
