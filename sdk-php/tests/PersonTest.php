<?php

declare(strict_types=1);

namespace Tests;

use PHPUnit\Framework\TestCase;
use Sdk\Person;

final class PersonTest extends TestCase
{
    public function testCalculator(): void
    {
        $person = new Person(5, 'John Doe', 'test@example.com');
        self::assertEquals(5, $person->getId());
        self::assertEquals('John Doe', $person->getName());
        self::assertEquals('test@example.com', $person->getEmail());
    }

    public function testEmailValidation(): void
    {
        $person = new Person(5, 'John Doe', 'test@example.com');
        self::assertTrue($person->validateEmail());

        $person = new Person(5, 'John Doe', 'testexample.com');
        self::assertFalse($person->validateEmail());
    }
}
