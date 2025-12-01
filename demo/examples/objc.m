// Objective-C example
#import <Foundation/Foundation.h>

@interface Person : NSObject

@property (nonatomic, strong) NSString *name;
@property (nonatomic, assign) NSInteger age;

- (instancetype)initWithName:(NSString *)name age:(NSInteger)age;
- (void)greet;

@end

@implementation Person

- (instancetype)initWithName:(NSString *)name age:(NSInteger)age {
    self = [super init];
    if (self) {
        _name = name;
        _age = age;
    }
    return self;
}

- (void)greet {
    NSLog(@"Hello, I'm %@ and I'm %ld years old", self.name, (long)self.age);
}

@end

int main(int argc, const char *argv[]) {
    @autoreleasepool {
        Person *person = [[Person alloc] initWithName:@"Alice" age:30];
        [person greet];
    }
    return 0;
}
