import { Component, signal, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { RouterModule, Router } from '@angular/router';
import { FortuneService, DailyFortuneDetail, FortuneStick } from '../_services/fortune.service';
import { PassportService } from '../_services/passport-service';

@Component({
  selector: 'app-fortune',
  templateUrl: './fortune.html',
  styleUrls: ['./fortune.scss'],
  standalone: true,
  imports: [CommonModule, MatButtonModule, RouterModule]
})
export class FortuneComponent implements OnInit {
  private fortuneService = inject(FortuneService);
  private passportService = inject(PassportService);
  private router = inject(Router);

  loading = signal(false);
  fortune = signal<DailyFortuneDetail | null>(null);
  
  // For anonymous users, we wrap the stick in a fake DailyFortuneDetail structure for display consistency
  // or handle it in the template. Let's make a unified signal or handling.
  // To keep template simple, I'll adapt the response.

  ngOnInit() {
  }

  async drawFortune() {
    this.loading.set(true);
    
    // Simulate shaking time
    await new Promise(resolve => setTimeout(resolve, 2000));

    try {
      if (this.passportService.isSignin()) {
        try {
          const result = await this.fortuneService.getDailyFortune();
          this.fortune.set(result);
        } catch (serverError) {
          console.warn('Server fortune failed, falling back to mock draw', serverError);
          this.useMockFortune();
        }
      } else {
        try {
          const stick = await this.fortuneService.drawRandomFortune();
          this.fortune.set({
            stick: stick,
            daily_fortune: {
              id: 0,
              user_id: 0,
              stick_id: stick.id,
              created_at: new Date().toISOString(),
              date: new Date().toLocaleDateString()
            }
          });
        } catch (serverError) {
          console.warn('Server anonymous fortune failed, falling back to mock draw', serverError);
          this.useMockFortune();
        }
      }
    } catch (error) {
      console.error('Fortune system error', error);
      alert('Failed to get fortune. Please try again.');
    } finally {
      this.loading.set(false);
    }
  }

  private useMockFortune() {
    const mockSticks: FortuneStick[] = [
      {
        id: 1,
        number: 8,
        poem_text: 'น้ำไหลจากเขาสูง สู่หุบเขาสงบ จิตใจปล่อยวาง ทุกข์มลายหายสิ้น',
        interpretation: 'ความพยายามที่ผ่านมาจะส่งผลในทางที่ดี ความวุ่นวายจะผ่านไป ความสงบจะเข้ามาแทนที่',
        lucky_direction: 'ตะวันออก'
      },
      {
        id: 2,
        number: 16,
        poem_text: 'พยัคฆ์ติดปีก บินสู่เวหา โอกาสมาถึง อย่าได้ลังเล',
        interpretation: 'งานใหญ่ที่คิดไว้จะมีคนคอยเกื้อหนุน การเงินจะดีขึ้นอย่างกะทันหัน',
        lucky_direction: 'เหนือ'
      },
      {
        id: 3,
        number: 25,
        poem_text: 'จันทร์สว่างกลางฟากฟ้า เมฆหมอกจางหาย ความจริงปรากฏ ทางออกชัดเจน',
        interpretation: 'ปัญหาที่เคยติดขัดจะคลี่คลายด้วยสติปัญญา มีเกณฑ์พบมิตรแท้คอยช่วยเหลือ',
        lucky_direction: 'ใต้'
      }
    ];
    
    const randomStick = mockSticks[Math.floor(Math.random() * mockSticks.length)];
    
    this.fortune.set({
      stick: randomStick,
      daily_fortune: {
        id: 0,
        user_id: 0,
        stick_id: randomStick.id,
        created_at: new Date().toISOString(),
        date: new Date().toLocaleDateString()
      }
    });
  }
}
